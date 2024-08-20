use std::rc::Rc;
use kernel::{
    Model, Entity, State, StateValue, Function,
    DictionaryState, EntityType, RelationType, DictionaryParameter,
    Process, AlwaysTrueCondition
};

fn main() {
    let mut model = Model::new();

    // Create the first entity (MovingEntity)
    let mut state = DictionaryState::new();
    state.set("position".to_string(), StateValue::Array(vec![
        StateValue::Integer(0),
        StateValue::Integer(0)
    ].into()));
    
    let moving_entity = Rc::new(Entity::new(
        "MovingEntity".to_string(),
        EntityType::Custom("MovingEntity".to_string()),
        Box::new(state)
    ));

    // Create a function for the moving entity
    let mut parameter = DictionaryParameter::new();
    parameter.set("speed".to_string(), StateValue::Integer(1));
    
    let move_function = Rc::new(Function::new(
        "MoveFunction".to_string(),
        Box::new(parameter),
        Rc::downgrade(&moving_entity)
    ));

    // Create a process for the move function
    let weak_function = Rc::downgrade(&move_function);
    let move_process = Process::new(
        "MoveProcess".to_string(),
        Box::new(AlwaysTrueCondition {}),
        weak_function.clone(),
        Box::new(move || {
            if let Some(function) = weak_function.upgrade() {
                if let Some(entity) = function.owner.upgrade() {
                    let mut state = entity.state.borrow_mut();
                    if let Some(StateValue::Array(position)) = state.get("position") {
                        if let (StateValue::Integer(x), StateValue::Integer(y)) = (&position[0], &position[1]) {
                            let new_position = StateValue::Array(vec![
                                StateValue::Integer(x + 1),
                                StateValue::Integer(y + 1)
                            ].into());
                            state.set("position".to_string(), new_position);
                        }
                    }
                }
            }
        })
    );

    // Add the process to the function
    move_function.add_process(move_process);

    // Add the function to the entity
    moving_entity.add_function(move_function);

    // Add the moving entity to the model
    model.add_entity(moving_entity.clone());

    // Create the second entity (StaticEntity)
    let mut state2 = DictionaryState::new();
    state2.set("name".to_string(), StateValue::String("StaticEntity".to_string()));
    
    let static_entity = Rc::new(Entity::new(
        "StaticEntity".to_string(),
        EntityType::Custom("StaticEntity".to_string()),
        Box::new(state2)
    ));

    // Add the static entity to the model
    model.add_entity(static_entity.clone());

    // Add a relation between entities
    moving_entity.add_relation(static_entity.id, RelationType::Custom("Observes".to_string()));

    // Run simulation
    model.simulate(10);

    // Print final state
    println!("Final model state: {}", model);

    // Print relations
    for entity in &model.entities {
        println!("Entity {} relations:", entity.name);
        for (relation_type, targets) in entity.get_relations() {
            println!("  {:?}: {:?}", relation_type, targets);
        }
    }
}
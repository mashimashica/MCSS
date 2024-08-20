use kernel::{
    Model, Entity, State, StateValue, Function, Process, Condition,
    DictionaryState, EntityType, RelationType, DictionaryParameter,
    SimpleProcess, AlwaysTrueCondition
};

fn main() {
    let mut model = Model::new();

    // Create an entity
    let mut state = DictionaryState::new();
    state.set("position".to_string(), StateValue::Array(vec![StateValue::Integer(0), StateValue::Integer(0)]));

    let move_condition = Box::new(AlwaysTrueCondition {});
    let move_process = Box::new(SimpleProcess {
        condition: move_condition,
        action: |entity| {
            let mut state = entity.state.borrow_mut();
            if let Some(StateValue::Array(position)) = state.get("position") {
                if let (StateValue::Integer(x), StateValue::Integer(y)) = (&position[0], &position[1]) {
                    let new_position = StateValue::Array(vec![
                        StateValue::Integer(x + 1),
                        StateValue::Integer(y + 1)
                    ]);
                    state.set("position".to_string(), new_position);
                }
            }
        },
    });

    let mut parameter = DictionaryParameter::new();
    parameter.set("speed".to_string(), StateValue::Integer(1));

    let function = Function {
        parameter: Box::new(parameter),
        processes: vec![move_process],
    };

    let mut entity = Entity::new(
        1,
        EntityType::Custom("MovingEntity".to_string()),
        Box::new(state)
    );
    entity.functions.push(function);

    model.add_entity(entity);

    // Create another entity and add a relation
    let mut state2 = DictionaryState::new();
    state2.set("name".to_string(), StateValue::String("Entity 2".to_string()));
    let entity2 = Entity::new(
        2,
        EntityType::Custom("StaticEntity".to_string()),
        Box::new(state2)
    );
    model.add_entity(entity2);

    // Add a relation between entities
    model.entities[0].add_relation(2, RelationType::Custom("Observes".to_string()));

    // Run simulation
    model.simulate(10);

    // Print final state
    println!("Final model state: {}", model);

    // Print relations
    for entity in &model.entities {
        println!("Entity {} relations:", entity.id);
        for (relation_type, targets) in entity.get_relations() {
            println!("  {:?}: {:?}", relation_type, targets);
        }
    }
}
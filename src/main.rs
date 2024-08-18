mod kernel;

use kernel::{
    Model, Entity, State, StateValue, Function, Process, Condition,
    DictionaryState, DictionaryParameter
};
use std::cell::RefCell;

fn main() {
    let mut model = Model::new();

    // Create an entity
    let mut state = DictionaryState::new();
    state.set("position".to_string(), StateValue::Array(vec![StateValue::Integer(0), StateValue::Integer(0)])); // 2D position

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

    let entity = Entity {
        id: 1,
        state: RefCell::new(Box::new(state)),
        functions: vec![function],
    };

    model.add_entity(entity);

    // Run simulation
    model.simulate(10);

    // Print final state
    println!("Final model state: {}", model);
}

#[derive(Debug)]
struct SimpleProcess {
    pub condition: Box<dyn Condition>,
    pub action: fn(&Entity),
}

impl Process for SimpleProcess {
    fn execute(&self, entity: &Entity) {
        (self.action)(entity);
    }

    fn check_condition(&self, entity: &Entity) -> bool {
        self.condition.is_met(entity)
    }
}

#[derive(Debug)]
struct AlwaysTrueCondition {}

impl Condition for AlwaysTrueCondition {
    fn is_met(&self, _entity: &Entity) -> bool {
        true
    }
}
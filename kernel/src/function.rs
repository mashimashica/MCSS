use std::cell::RefCell;
use std::rc::Weak;
use std::fmt;
use std::collections::HashMap;
use uuid::Uuid;
use crate::state::StateValue;
use crate::process::Process;
use crate::entity::Entity;

#[derive(Debug)]
pub struct Function {
    // pub id: Uuid,
    pub name: String,
    pub parameter: RefCell<Parameter>,
    pub processes: RefCell<Vec<Process>>,
    pub owner: Weak<Entity>,
}

impl Function {
    pub fn new(name: String, parameter: Parameter, owner: Weak<Entity>) -> Self {
        Function {
            // id: Uuid::new_v4(),
            name,
            parameter: RefCell::new(parameter),
            processes: RefCell::new(Vec::new()),
            owner,
        }
    }

    pub fn add_process(&self, process: Process) {
        self.processes.borrow_mut().push(process);
    }
}

// pub trait Parameter: fmt::Debug {
//     fn get(&self, key: &str) -> Option<&StateValue>;
//     fn set(&mut self, key: String, value: StateValue);
// }

#[derive(Debug)]
pub struct Parameter {
    values: HashMap<String, StateValue>,
}

impl Parameter {
    pub fn new() -> Self {
        Parameter {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: StateValue) {
        self.values.insert(key, value);
    }

    pub fn get(&self, key: &str) -> Option<&StateValue> {
        self.values.get(key)
    }
}
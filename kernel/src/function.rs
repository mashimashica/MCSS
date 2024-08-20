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
    pub id: Uuid,
    pub name: String,
    pub parameter: Box<dyn Parameter>,
    pub processes: RefCell<Vec<Process>>,
    pub owner: Weak<Entity>,
}

impl Function {
    pub fn new(name: String, parameter: Box<dyn Parameter>, owner: Weak<Entity>) -> Self {
        Function {
            id: Uuid::new_v4(),
            name,
            parameter,
            processes: RefCell::new(Vec::new()),
            owner,
        }
    }

    pub fn add_process(&self, process: Process) {
        self.processes.borrow_mut().push(process);
    }
}

pub trait Parameter: fmt::Debug {
    fn get(&self, key: &str) -> Option<&StateValue>;
}

#[derive(Debug)]
pub struct DictionaryParameter {
    values: HashMap<String, StateValue>,
}

impl DictionaryParameter {
    pub fn new() -> Self {
        DictionaryParameter {
            values: HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: StateValue) {
        self.values.insert(key, value);
    }
}

impl Parameter for DictionaryParameter {
    fn get(&self, key: &str) -> Option<&StateValue> {
        self.values.get(key)
    }
}
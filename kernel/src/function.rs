use std::collections::HashMap;
use std::fmt;
use crate::state::StateValue;
use crate::process::Process;

#[derive(Debug)]
pub struct Function {
    pub parameter: Box<dyn Parameter>,
    pub processes: Vec<Box<dyn Process>>,
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
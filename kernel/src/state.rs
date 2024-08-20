use std::collections::HashMap;
use std::fmt;

pub trait State: fmt::Debug {
    fn get(&self, key: &str) -> Option<&StateValue>;
    fn set(&mut self, key: String, value: StateValue);
}

#[derive(Debug)]
pub struct DictionaryState {
    values: HashMap<String, StateValue>,
}

impl DictionaryState {
    pub fn new() -> Self {
        DictionaryState {
            values: HashMap::new(),
        }
    }
}

impl State for DictionaryState {
    fn get(&self, key: &str) -> Option<&StateValue> {
        self.values.get(key)
    }

    fn set(&mut self, key: String, value: StateValue) {
        self.values.insert(key, value);
    }
}

#[derive(Debug, Clone)]
pub enum StateValue {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Array(Vec<StateValue>),
}
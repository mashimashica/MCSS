use std::collections::HashMap;

#[derive(Debug)]
pub struct Variable {
    values: HashMap<String, Value>,
}

impl Variable {
    pub fn new() -> Self {
        Variable {
            values: HashMap::new(),
        }
    }

    pub fn get(&self, key: &str) -> Option<&Value> {
        self.values.get(key)
    }

    pub fn set(&mut self, key: String, value: Value) {
        self.values.insert(key, value);
    }

    pub fn remove(&mut self, key: &str) {
        self.values.remove(key);
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<'_, String, Value> {
        self.values.iter()
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Integer(i32),
    Float(f32),
    String(String),
    Boolean(bool),
    Array(Vec<Value>),
}
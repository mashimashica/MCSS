use std::collections::HashMap;
use std::fmt;
use std::cell::RefCell;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum EntityType {
    Person,
    Household,
    Organization,
    Location,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum RelationType {
    Friend,
    Family,
    Colleague,
    Custom(String),
}

#[derive(Debug)]
pub struct Entity {
    pub id: u32,
    pub entity_type: EntityType,
    pub state: RefCell<Box<dyn State>>,
    pub functions: Vec<Function>,
    pub relations: RefCell<HashMap<RelationType, Vec<u32>>>,
}

impl Entity {
    pub fn new(id: u32, entity_type: EntityType, state: Box<dyn State>) -> Self {
        Entity {
            id,
            entity_type,
            state: RefCell::new(state),
            functions: Vec::new(),
            relations: RefCell::new(HashMap::new()),
        }
    }

    pub fn add_relation(&self, target: u32, relation_type: RelationType) {
        self.relations.borrow_mut()
            .entry(relation_type)
            .or_insert_with(Vec::new)
            .push(target);
    }

    pub fn remove_relation(&self, target: u32, relation_type: &RelationType) {
        if let Some(targets) = self.relations.borrow_mut().get_mut(relation_type) {
            targets.retain(|&t| t != target);
            if targets.is_empty() {
                self.relations.borrow_mut().remove(relation_type);
            }
        }
    }

    pub fn get_relations(&self) -> HashMap<RelationType, Vec<u32>> {
        self.relations.borrow().clone()
    }

    pub fn get_relations_by_type(&self, relation_type: &RelationType) -> Option<Vec<u32>> {
        self.relations.borrow().get(relation_type).cloned()
    }
}

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

pub trait Condition: fmt::Debug {
    fn is_met(&self, entity: &Entity) -> bool;
}

pub trait Process: fmt::Debug {
    fn execute(&self, entity: &Entity);
    fn check_condition(&self, entity: &Entity) -> bool;
}

pub struct Model {
    pub entities: Vec<Entity>,
    pub time: u32,
}

impl Model {
    pub fn new() -> Self {
        Model {
            entities: Vec::new(),
            time: 0,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    fn proceed(&mut self) {
        for entity in &self.entities {
            let mut to_execute = Vec::new();
            
            for function in &entity.functions {
                for process in &function.processes {
                    if process.check_condition(entity) {
                        to_execute.push(process);
                    }
                }
            }
            
            for process in to_execute {
                process.execute(entity);
            }
        }
        self.time += 1;
    }

    pub fn simulate(&mut self, steps: u32) {
        for _ in 0..steps {
            self.proceed();
        }
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Simulation time: {}", self.time)?;
        for (i, entity) in self.entities.iter().enumerate() {
            writeln!(f, "Entity {}: {:?}", i, entity)?;
        }
        Ok(())
    }
}
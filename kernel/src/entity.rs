use std::collections::HashMap;
use std::cell::RefCell;
use crate::state::{State, StateValue};
use crate::function::Function;
use crate::types::{EntityType, RelationType};

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
use std::collections::HashMap;
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use uuid::Uuid;
use crate::relation::Relation;
use crate::types::EntityType;
use crate::variable::Variable;
use crate::function::Function;

#[derive(Debug)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub entity_type: EntityType,
    pub state: RefCell<Variable>,
    pub functions: RefCell<HashMap<String, Rc<Function>>>,
    pub relations: RefCell<HashMap<String, Vec<Weak<Relation>>>>,
}

impl Entity {
    pub fn new(name: String, entity_type: EntityType) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            entity_type,
            state: RefCell::new(Variable::new()),
            functions: RefCell::new(HashMap::new()),
            relations: RefCell::new(HashMap::new()),
        }
    }

    pub fn get_state(&self) -> &RefCell<Variable> {
        &self.state
    }

    pub fn add_function(&self, function: Rc<Function>) {
        self.functions.borrow_mut().insert(function.name.clone(), function);
    }

    pub fn get_function(&self, name: &str) -> Option<Rc<Function>> {
        self.functions.borrow().get(name).cloned()
    }

    pub fn remove_function(&self, name: &str) -> Option<Rc<Function>> {
        self.functions.borrow_mut().remove(name)
    }

    pub fn get_relations(&self, name: &str) -> Vec<Rc<Relation>> {
        self.relations.borrow()
            .get(name)
            .map(|vec| vec.iter().filter_map(Weak::upgrade).collect())
            .unwrap_or_else(Vec::new)
    }

    pub fn get_all_relations(&self) -> Vec<Rc<Relation>> {
        self.relations.borrow()
            .values()
            .flatten()
            .filter_map(Weak::upgrade)
            .collect()
    }

    pub(crate) fn add_relation(&self, name: String, relation: Weak<Relation>) {
        self.relations.borrow_mut()
            .entry(name)
            .or_insert_with(Vec::new)
            .push(relation);
    }

    pub(crate) fn remove_relation(&self, name: &str, relation_id: Uuid) {
        let mut relations = self.relations.borrow_mut();
        if let Some(rel_vec) = relations.get_mut(name) {
            rel_vec.retain(|r| r.upgrade().map(|rc| rc.id) != Some(relation_id));
            if rel_vec.is_empty() {
                relations.remove(name);
            }
        }
    }
}
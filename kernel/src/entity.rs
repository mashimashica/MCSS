use std::collections::HashMap;
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;
use crate::state::State;
use crate::function::Function;
use crate::types::{EntityType, RelationType};

#[derive(Debug)]
pub struct Entity {
    pub id: Uuid,
    pub name: String,
    pub entity_type: EntityType,
    pub state: RefCell<State>,
    pub functions: RefCell<Vec<Rc<Function>>>,
    pub relations: RefCell<HashMap<RelationType, Vec<Uuid>>>,
}

impl Entity {
    pub fn new(name: String, entity_type: EntityType, state: State) -> Rc<Self> {
        Rc::new(Entity {
            id: Uuid::new_v4(),
            name,
            entity_type,
            state: RefCell::new(state),
            functions: RefCell::new(Vec::new()),
            relations: RefCell::new(HashMap::new()),
        })
    }

    pub fn add_function(&self, function: Rc<Function>) {
        self.functions.borrow_mut().push(function);
    }

    pub fn add_relation(&self, target: Uuid, relation_type: RelationType) {
        self.relations.borrow_mut()
            .entry(relation_type)
            .or_insert_with(Vec::new)
            .push(target);
    }

    pub fn remove_relation(&self, target: Uuid, relation_type: &RelationType) {
        if let Some(targets) = self.relations.borrow_mut().get_mut(relation_type) {
            targets.retain(|&t| t != target);
            if targets.is_empty() {
                self.relations.borrow_mut().remove(relation_type);
            }
        }
    }

    pub fn get_relations(&self) -> HashMap<RelationType, Vec<Uuid>> {
        self.relations.borrow().clone()
    }

    pub fn get_relations_by_type(&self, relation_type: &RelationType) -> Option<Vec<Uuid>> {
        self.relations.borrow().get(relation_type).cloned()
    }
}
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use std::collections::HashMap;
use uuid::Uuid;
use crate::entity::Entity;
use crate::types::{EntityType, RelationType};
use crate::variable::{Variable, Value};
use crate::context::{ReadOnlyRelation, ReadOnlyEntity};

#[derive(Debug)]
pub struct Relation {
    pub id: Uuid,
    pub name: String,
    pub relation_type: RelationType,
    pub entity1: Weak<Entity>,
    pub entity2: Weak<Entity>,
    pub meta: RefCell<Variable>,
}

impl Relation {
    pub fn new(name: String, relation_type: RelationType, entity1: Weak<Entity>, entity2: Weak<Entity>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            relation_type,
            entity1,
            entity2,
            meta: RefCell::new(Variable::new()),
        }
    }

    pub fn get_other_entity(&self, entity: &Entity) -> Option<Weak<Entity>> {
        if self.entity1.upgrade().map(|e| e.id) == Some(entity.id) {
            Some(self.entity2.clone())
        } else {
            Some(self.entity1.clone())
        }
    }

    pub fn get_type(&self) -> &RelationType {
        &self.relation_type
    }

    pub fn get_meta(&self) -> &RefCell<Variable> {
        &self.meta
    }

    pub fn add_metadata(&self, key: String, value: Value) {
        self.meta.borrow_mut().set(key, value);
    }

    pub fn remove_metadata(&self, key: &str) {
        self.meta.borrow_mut().remove(key);
    }

    pub fn get_meta_value(&self, key: &str) -> Option<Value> {
        self.meta.borrow().get(key).cloned()
    }

    pub fn iter_meta(&self) -> HashMap<String, Value> {
        self.meta.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

impl ReadOnlyRelation for Relation {
    fn get_id(&self) -> Uuid {
        self.id
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn get_relation_type(&self) -> &RelationType {
        &self.relation_type
    }

    fn get_entity1(&self) -> Option<Rc<dyn ReadOnlyEntity>> {
        self.entity1.upgrade().map(|e| e as Rc<dyn ReadOnlyEntity>)
    }

    fn get_entity2(&self) -> Option<Rc<dyn ReadOnlyEntity>> {
        self.entity2.upgrade().map(|e| e as Rc<dyn ReadOnlyEntity>)
    }

    fn get_meta_value(&self, key: &str) -> Option<Value> {
        self.get_meta_value(key)
    }

    fn iter_meta(&self) -> HashMap<String, Value> {
        self.meta.borrow().iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }
}

#[derive(Debug, Clone)]
pub struct RelationshipDefinition {
    pub name: String,
    pub source_type: EntityType,
    pub target_type: EntityType,
    pub relation_type: RelationType,
}

#[derive(Debug)]
pub struct RelationshipRegistry {
    pub definitions: HashMap<String, RelationshipDefinition>,
}

impl RelationshipRegistry {
    pub fn new() -> Self {
        Self {
            definitions: HashMap::new(),
        }
    }

    pub fn add_definition(&mut self, definition: RelationshipDefinition) {
        self.definitions.insert(definition.name.clone(), definition);
    }

    pub fn get_definition(&self, name: &str) -> Option<&RelationshipDefinition> {
        self.definitions.get(name)
    }

    pub fn remove_definition(&mut self, name: &str) -> Option<RelationshipDefinition> {
        self.definitions.remove(name)
    }

    pub fn iter_definitions(&self) -> std::collections::hash_map::Iter<'_, String, RelationshipDefinition> {
        self.definitions.iter()
    }
}
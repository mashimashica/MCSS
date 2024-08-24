use std::rc::Rc;
use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use uuid::Uuid;
use crate::types::{EntityType, RelationType};
use crate::variable::{Variable, Value};

pub trait ReadOnlyEntity {
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> &str;
    fn get_entity_type(&self) -> &EntityType;
    fn get_state(&self) -> Ref<Variable>;
    fn get_function(&self, name: &str) -> Option<Rc<dyn ReadOnlyFunction>>;
    fn get_relations(&self, name: &str) -> Vec<Rc<dyn ReadOnlyRelation>>;
}

pub trait ReadOnlyFunction {
    fn get_name(&self) -> &str;
    fn get_parameter(&self) -> &RefCell<Variable>;
    fn is_active(&self) -> bool;
}

pub trait ReadOnlyRelation {
    fn get_id(&self) -> Uuid;
    fn get_name(&self) -> &str;
    fn get_relation_type(&self) -> &RelationType;
    fn get_entity1(&self) -> Option<Rc<dyn ReadOnlyEntity>>;
    fn get_entity2(&self) -> Option<Rc<dyn ReadOnlyEntity>>;
    fn get_meta_value(&self, key: &str) -> Option<Value>;
    fn iter_meta(&self) -> HashMap<String, Value>; // 変更: イテレータではなくHashMapを返す
}

pub trait ReadOnlyModel {
    fn get_entity(&self, id: &Uuid) -> Option<Rc<dyn ReadOnlyEntity>>;
    fn get_all_entities(&self) -> Vec<Rc<dyn ReadOnlyEntity>>;
    fn get_entities_by_type(&self, entity_type: &EntityType) -> Vec<Rc<dyn ReadOnlyEntity>>;
    fn get_entities_by_name(&self, name: &str) -> Vec<Rc<dyn ReadOnlyEntity>>;
    fn get_relation(&self, id: &Uuid) -> Option<Rc<dyn ReadOnlyRelation>>;
    fn get_all_relations(&self) -> Vec<Rc<dyn ReadOnlyRelation>>;
}

pub struct ExecutionContext<'a> {
    pub owner_function: &'a dyn ReadOnlyFunction,
    pub owner_entity: &'a dyn ReadOnlyEntity,
    pub model: &'a dyn ReadOnlyModel,
}
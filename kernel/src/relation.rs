use std::rc::Weak;
use uuid::Uuid;
use crate::entity::Entity;
use crate::types::RelationType;

#[derive(Debug)]
pub struct Relation {
    pub id: Uuid,
    pub name: String,
    pub relation_type: RelationType,
    pub entity1: Weak<Entity>,
    pub entity2: Weak<Entity>,
}

impl Relation {
    pub fn new(name: String, relation_type: RelationType, entity1: Weak<Entity>, entity2: Weak<Entity>) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            relation_type,
            entity1,
            entity2,
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
}
use std::rc::Weak;
use uuid::Uuid;
use crate::entity::Entity;
use crate::types::RelationType;
use crate::context::ReadOnlyRelation;
use crate::context::ReadOnlyEntity;
use std::rc::Rc;

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

// ReadOnlyRelation トレイトの実装
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
}
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use uuid::Uuid;
use crate::entity::Entity;
use crate::relation::Relation;
use crate::types::{EntityType, RelationType};

#[derive(Debug)]
pub enum ModelError {
    EntityNotFound(Uuid),
    RelationNotFound(Uuid),
    RelationAlreadyExists(String),
    InvalidRelationType { name: String, relation_type: RelationType },
}

pub struct Model {
    entities: RefCell<HashMap<Uuid, Rc<Entity>>>,
    relations: RefCell<HashMap<Uuid, Rc<Relation>>>,
    time: u32,
}

impl Model {
    pub fn new() -> Self {
        Self {
            entities: RefCell::new(HashMap::new()),
            relations: RefCell::new(HashMap::new()),
            time: 0,
        }
    }

    pub fn create_entity(&self, name: String, entity_type: EntityType) -> Rc<Entity> {
        let entity = Rc::new(Entity::new(name, entity_type));
        self.entities.borrow_mut().insert(entity.id, entity.clone());
        entity
    }

    pub fn get_entity(&self, id: &Uuid) -> Option<Rc<Entity>> {
        self.entities.borrow().get(id).cloned()
    }

    pub fn add_relation(
        &self,
        name: String,
        relation_type: RelationType,
        entity1_id: &Uuid,
        entity2_id: &Uuid,
    ) -> Result<Rc<Relation>, ModelError> {
        let entities = self.entities.borrow();
        let entity1 = entities.get(entity1_id).ok_or(ModelError::EntityNotFound(*entity1_id))?;
        let entity2 = entities.get(entity2_id).ok_or(ModelError::EntityNotFound(*entity2_id))?;

        // Check RelationType constraints
        match relation_type {
            RelationType::OneToOne => {
                if !entity1.get_relations(&name).is_empty() || !entity2.get_relations(&name).is_empty() {
                    return Err(ModelError::InvalidRelationType { name, relation_type });
                }
            },
            RelationType::OneToMany => {
                if !entity1.get_relations(&name).is_empty() {
                    return Err(ModelError::InvalidRelationType { name, relation_type });
                }
            },
            RelationType::ManyToOne => {
                if !entity2.get_relations(&name).is_empty() {
                    return Err(ModelError::InvalidRelationType { name, relation_type });
                }
            },
            RelationType::ManyToMany => {} // No constraints
        }

        let relation = Rc::new(Relation::new(
            name.clone(),
            relation_type,
            Rc::downgrade(entity1),
            Rc::downgrade(entity2),
        ));

        entity1.add_relation(name.clone(), Rc::downgrade(&relation));
        entity2.add_relation(name, Rc::downgrade(&relation));

        self.relations.borrow_mut().insert(relation.id, relation.clone());

        Ok(relation)
    }

    pub fn remove_relation(&self, relation_id: &Uuid) -> Result<(), ModelError> {
        let mut relations = self.relations.borrow_mut();
        let relation = relations.remove(relation_id).ok_or(ModelError::RelationNotFound(*relation_id))?;

        if let Some(entity1) = relation.entity1.upgrade() {
            entity1.remove_relation(&relation.name, relation.id);
        }
        if let Some(entity2) = relation.entity2.upgrade() {
            entity2.remove_relation(&relation.name, relation.id);
        }

        Ok(())
    }

    pub fn get_relation(&self, id: &Uuid) -> Option<Rc<Relation>> {
        self.relations.borrow().get(id).cloned()
    }

    pub fn get_all_entities(&self) -> Vec<Rc<Entity>> {
        self.entities.borrow().values().cloned().collect()
    }

    pub fn get_all_relations(&self) -> Vec<Rc<Relation>> {
        self.relations.borrow().values().cloned().collect()
    }

    pub fn proceed(&mut self) {
        // Implement simulation logic here
        self.time += 1;
    }

    pub fn simulate(&mut self, steps: u32) {
        for _ in 0..steps {
            self.proceed();
        }
    }
}
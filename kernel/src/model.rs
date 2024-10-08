use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use uuid::Uuid;
use crate::entity::Entity;
use crate::relation::{Relation, RelationshipDefinition, RelationshipRegistry};
use crate::types::{EntityType, RelationType};
use crate::context::{ExecutionContext, ReadOnlyRelation, ReadOnlyModel, ReadOnlyEntity};
use crate::result::{ExecutionResult, EntityCreationInfo, RelationCreationInfo, FunctionCreationInfo, ProcessCreationInfo};
use crate::process::{Process, Condition};
use crate::function::Function;
use crate::variable::Value;

#[derive(Debug)]
pub enum ModelError {
    EntityNotFound(Uuid),
    RelationNotFound(Uuid),
    RelationAlreadyExists(String),
    InvalidRelationType { name: String, relation_type: RelationType },
    UndefinedRelation(String),
    InvalidRelationEntityTypes,
}

pub struct Model {
    entities: RefCell<HashMap<Uuid, Rc<Entity>>>,
    relations: RefCell<HashMap<Uuid, Rc<Relation>>>,
    relationship_registry: RefCell<RelationshipRegistry>,
    processes: RefCell<Vec<Rc<Process>>>,
}

impl Model {
    pub fn new() -> Self {
        Self {
            entities: RefCell::new(HashMap::new()),
            relations: RefCell::new(HashMap::new()),
            relationship_registry: RefCell::new(RelationshipRegistry::new()),
            processes: RefCell::new(Vec::new()),
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

    pub fn get_all_entities(&self) -> Vec<Rc<Entity>> {
        self.entities.borrow().values().cloned().collect()
    }

    pub fn get_entities_by_type(&self, entity_type: &EntityType) -> Vec<Rc<Entity>> {
        self.entities
            .borrow()
            .values()
            .filter(|entity| entity.entity_type == *entity_type)
            .cloned()
            .collect()
    }

    pub fn get_entities_by_name(&self, name: &str) -> Vec<Rc<Entity>> {
        self.entities
            .borrow()
            .values()
            .filter(|entity| entity.name == name)
            .cloned()
            .collect()
    }

    pub fn get_entities_by_name_prefix(&self, prefix: &str) -> Vec<Rc<Entity>> {
        self.entities
            .borrow()
            .values()
            .filter(|entity| entity.name.starts_with(prefix))
            .cloned()
            .collect()
    }

    pub fn get_all_entity_types(&self) -> Vec<EntityType> {
        self.entities
            .borrow()
            .values()
            .map(|entity| entity.entity_type.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect()
    }

    pub fn define_relationship(
        &self,
        name: String,
        source_type: EntityType,
        target_type: EntityType,
        relation_type: RelationType,
    ) -> Result<(), String> {
        let definition = RelationshipDefinition {
            name: name.clone(),
            source_type,
            target_type,
            relation_type,
        };
        self.relationship_registry.borrow_mut().add_definition(definition);
        Ok(())
    }

    pub fn add_relation(
        &self,
        name: String,
        entity1_id: &Uuid,
        entity2_id: &Uuid,
    ) -> Result<Rc<Relation>, ModelError> {
        let registry = self.relationship_registry.borrow();
        let definition = registry.get_definition(&name)
            .ok_or(ModelError::UndefinedRelation(name.clone()))?;

        let entity1 = self.get_entity(entity1_id)
            .ok_or(ModelError::EntityNotFound(*entity1_id))?;
        let entity2 = self.get_entity(entity2_id)
            .ok_or(ModelError::EntityNotFound(*entity2_id))?;

        if entity1.entity_type != definition.source_type || 
           entity2.entity_type != definition.target_type {
            return Err(ModelError::InvalidRelationEntityTypes);
        }

        match definition.relation_type {
            RelationType::OneToOne => {
                if !entity1.get_relations(&name).is_empty() || !entity2.get_relations(&name).is_empty() {
                    return Err(ModelError::InvalidRelationType { name, relation_type: definition.relation_type });
                }
            },
            RelationType::OneToMany => {
                if !entity1.get_relations(&name).is_empty() {
                    return Err(ModelError::InvalidRelationType { name, relation_type: definition.relation_type });
                }
            },
            RelationType::ManyToOne => {
                if !entity2.get_relations(&name).is_empty() {
                    return Err(ModelError::InvalidRelationType { name, relation_type: definition.relation_type });
                }
            },
            RelationType::ManyToMany => {} // No constraints
        }

        let relation = Rc::new(Relation::new(
            name.clone(),
            definition.relation_type,
            Rc::downgrade(&entity1),
            Rc::downgrade(&entity2),
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

    pub fn get_all_relations(&self) -> Vec<Rc<Relation>> {
        self.relations.borrow().values().cloned().collect()
    }

    // プロセスを追加するメソッド
    pub fn add_process(&self, process: Rc<Process>) {
        self.processes.borrow_mut().push(process);
    }

    // シミュレーター機能
    pub fn simulate(&self) {
        let mut results = Vec::new();
        
        // モデルレベルのプロセスを実行
        for process in self.processes.borrow().iter() {
            if let Some(function) = process.owner.upgrade() {
                if let Some(entity) = function.owner.upgrade() {
                    let context = ExecutionContext {
                        owner_function: &*function,
                        owner_entity: &*entity,
                        model: self,
                    };
                    let process_results = process.execute(&context);
                    results.extend(process_results);
                }
            }
        }
        self.apply_results(results);
    }

    fn apply_results(&self, results: Vec<ExecutionResult>) {
        for result in results {
            match result {
                ExecutionResult::CreateEntity(info) => {
                    self.create_entity_internal(info);
                }
                ExecutionResult::DeleteEntity(id) => {
                    self.delete_entity_internal(id);
                }
                ExecutionResult::CreateRelation(info) => {
                    self.create_relation_internal(info, None);
                }
                ExecutionResult::DeleteRelation(id) => {
                    self.delete_relation_internal(id);
                }
                ExecutionResult::AddFunction(entity_id, function_info) => {
                    self.add_function_internal(entity_id, function_info);
                }
                ExecutionResult::RemoveFunction(entity_id, function_name) => {
                    self.remove_function_internal(entity_id, function_name);
                }
                ExecutionResult::ActivateFunction(entity_id, function_name) => {
                    self.activate_function_internal(entity_id, function_name);
                }
                ExecutionResult::DeactivateFunction(entity_id, function_name) => {
                    self.deactivate_function_internal(entity_id, function_name);
                }
                ExecutionResult::AddProcess(entity_id, function_name, process_info) => {
                    self.add_process_internal(entity_id, function_name, process_info);
                }
                ExecutionResult::RemoveProcess(entity_id, function_name, process_name) => {
                    self.remove_process_internal(entity_id, function_name, process_name);
                }
                ExecutionResult::UpdateEntityState(entity_id, key, value) => {
                    self.update_entity_state_internal(entity_id, key, value);
                }
                ExecutionResult::DeleteEntityState(entity_id, key) => {
                    self.delete_entity_state_internal(entity_id, key);
                }
                ExecutionResult::UpdateFunctionParameter(entity_id, function_name, key, value) => {
                    self.update_function_parameter_internal(entity_id, function_name, key, value);
                }
                ExecutionResult::DeleteFunctionParameter(entity_id, function_name, key) => {
                    self.delete_function_parameter_internal(entity_id, function_name, key);
                }
                ExecutionResult::AddCondition(entity_id, function_name, process_name, condition) => {
                    self.add_condition_internal(entity_id, function_name, process_name, condition);
                }
                ExecutionResult::RemoveCondition(entity_id, function_name, process_name) => {
                    self.remove_condition_internal(entity_id, function_name, process_name);
                }
                ExecutionResult::AddRelationMetadata(relation_id, key, value) => {
                    self.add_relation_metadata_internal(relation_id, key, value);
                }
                ExecutionResult::RemoveRelationMetadata(relation_id, key) => {
                    self.remove_relation_metadata_internal(relation_id, key);
                }
            }
        }
    }
    
    fn create_entity_internal(&self, info: EntityCreationInfo) -> Rc<Entity> {
        let entity = Rc::new(Entity::new(info.name, info.entity_type));
        
        for (key, value) in info.initial_state {
            entity.get_state().borrow_mut().set(key, value);
        }

        for function_info in info.functions {
            self.add_function_internal(entity.id, function_info);
        }

        for relation_info in info.relations {
            self.create_relation_internal(relation_info, Some(entity.id));
        }

        self.entities.borrow_mut().insert(entity.id, Rc::clone(&entity));
        entity
    }

    fn delete_entity_internal(&self, id: Uuid) {
        if let Some(entity) = self.entities.borrow_mut().remove(&id) {
            let relations_to_remove: Vec<Uuid> = self.relations.borrow()
                .values()
                .filter(|r| r.entity1.upgrade().map(|e| e.id) == Some(id) || r.entity2.upgrade().map(|e| e.id) == Some(id))
                .map(|r| r.id)
                .collect();
            
            for relation_id in relations_to_remove {
                self.delete_relation_internal(relation_id);
            }

            self.processes.borrow_mut().retain(|process| {
                process.owner.upgrade()
                    .and_then(|f| f.owner.upgrade())
                    .map(|e| e.id != id)
                    .unwrap_or(true)
            });

            for function in entity.get_all_functions() {
                self.remove_function_internal(id, function.name.clone());
            }
        }
    }

    fn create_relation_internal(&self, info: RelationCreationInfo, source_entity_id: Option<Uuid>) {
        let registry = self.relationship_registry.borrow();
        let definition = registry.get_definition(&info.name)
            .expect("Relation definition not found");
        let source_id = source_entity_id.unwrap_or(info.target_entity_id.unwrap());
        let target_id = info.target_entity_id.unwrap_or_else(|| {
            self.entities.borrow().values()
                .find(|e| e.name == *info.target_entity_name.as_ref().unwrap())
                .map(|e| e.id)
                .expect("Target entity not found")
        });

        let relation = Rc::new(Relation::new(
            info.name.clone(),
            definition.relation_type,
            Rc::downgrade(&self.entities.borrow()[&source_id]),
            Rc::downgrade(&self.entities.borrow()[&target_id])
        ));

        self.relations.borrow_mut().insert(relation.id, Rc::clone(&relation));

        self.entities.borrow()[&source_id].add_relation(info.name.clone(), Rc::downgrade(&relation));
        self.entities.borrow()[&target_id].add_relation(info.name, Rc::downgrade(&relation));

        if let Some(metadata) = info.metadata {
            for (key, value) in metadata {
                self.add_relation_metadata_internal(relation.id, key, value);
            }
        }
    }

    fn delete_relation_internal(&self, id: Uuid) {
        if let Some(relation) = self.relations.borrow_mut().remove(&id) {
            if let Some(entity1) = relation.entity1.upgrade() {
                entity1.remove_relation(&relation.name, relation.id);
            }
            if let Some(entity2) = relation.entity2.upgrade() {
                entity2.remove_relation(&relation.name, relation.id);
            }
        }
    }

    fn add_function_internal(&self, entity_id: Uuid, function_info: FunctionCreationInfo) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            let function = Rc::new(Function::new(
                function_info.name.clone(),
                Rc::downgrade(entity)
            ));

            for (key, value) in function_info.initial_parameters {
                function.get_parameter().borrow_mut().set(key, value);
            }

            for process_info in function_info.processes {
                self.add_process_internal(entity_id, function.name.clone(), process_info);
            }

            entity.add_function(Rc::clone(&function));
        }
    }

    fn remove_function_internal(&self, entity_id: Uuid, function_name: String) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if entity.remove_function(&function_name).is_some() {
                self.processes.borrow_mut().retain(|process| {
                    process.owner.upgrade().map(|f| f.name != function_name).unwrap_or(true)
                });
            }
        }
    }

    fn activate_function_internal(&self, entity_id: Uuid, function_name: String) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if let Some(function) = entity.get_function(&function_name) {
                function.activate();
            }
        }
    }

    fn deactivate_function_internal(&self, entity_id: Uuid, function_name: String) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if let Some(function) = entity.get_function(&function_name) {
                function.deactivate();
            }
        }
    }

    fn add_process_internal(&self, entity_id: Uuid, function_name: String, process_info: ProcessCreationInfo) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if let Some(function) = entity.get_function(&function_name) {
                let process = Rc::new(Process::new(
                    process_info.name,
                    Rc::downgrade(&function),
                    process_info.action
                ));
                function.add_process(Rc::clone(&process));
                self.add_process(Rc::clone(&process));

                if let Some(condition) = process_info.condition {
                    self.add_condition_internal(entity_id, function_name, process.name.clone(), condition);
                }
            }
        }
    }

    fn remove_process_internal(&self, entity_id: Uuid, function_name: String, process_name: String) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if let Some(function) = entity.get_function(&function_name) {
                function.remove_process(&process_name);
                self.processes.borrow_mut().retain(|p| p.name != process_name);
            }
        }
    }

    fn update_entity_state_internal(&self, entity_id: Uuid, key: String, value: Value) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            entity.get_state().borrow_mut().set(key, value);
        }
    }

    fn delete_entity_state_internal(&self, entity_id: Uuid, key: String) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            entity.get_state().borrow_mut().remove(&key);
        }
    }

    fn update_function_parameter_internal(&self, entity_id: Uuid, function_name: String, key: String, value: Value) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if let Some(function) = entity.get_function(&function_name) {
                function.get_parameter().borrow_mut().set(key, value);
            }
        }
    }

    fn delete_function_parameter_internal(&self, entity_id: Uuid, function_name: String, key: String) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if let Some(function) = entity.get_function(&function_name) {
                function.get_parameter().borrow_mut().remove(&key);
            }
        }
    }

    fn add_condition_internal(&self, entity_id: Uuid, function_name: String, process_name: String, condition: Box<dyn Condition>) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if let Some(function) = entity.get_function(&function_name) {
                if let Some(process) = function.get_process(&process_name) {
                    process.set_condition(condition);
                }
            }
        }
    }

    fn remove_condition_internal(&self, entity_id: Uuid, function_name: String, process_name: String) {
        if let Some(entity) = self.entities.borrow().get(&entity_id) {
            if let Some(function) = entity.get_function(&function_name) {
                if let Some(process) = function.get_process(&process_name) {
                    process.remove_condition();
                }
            }
        }
    }

    fn add_relation_metadata_internal(&self, relation_id: Uuid, key: String, value: Value) {
        if let Some(relation) = self.relations.borrow().get(&relation_id) {
            relation.add_metadata(key, value);
        }
    }

    fn remove_relation_metadata_internal(&self, relation_id: Uuid, key: String) {
        if let Some(relation) = self.relations.borrow().get(&relation_id) {
            relation.remove_metadata(&key);
        }
    }
}

impl ReadOnlyModel for Model {
    fn get_entity(&self, id: &Uuid) -> Option<Rc<dyn ReadOnlyEntity>> {
        self.get_entity(id).map(|e| e as Rc<dyn ReadOnlyEntity>)
    }

    fn get_all_entities(&self) -> Vec<Rc<dyn ReadOnlyEntity>> {
        self.get_all_entities().into_iter().map(|e| e as Rc<dyn ReadOnlyEntity>).collect()
    }

    fn get_entities_by_type(&self, entity_type: &EntityType) -> Vec<Rc<dyn ReadOnlyEntity>> {
        self.get_entities_by_type(entity_type).into_iter().map(|e| e as Rc<dyn ReadOnlyEntity>).collect()
    }

    fn get_entities_by_name(&self, name: &str) -> Vec<Rc<dyn ReadOnlyEntity>> {
        self.get_entities_by_name(name).into_iter().map(|e| e as Rc<dyn ReadOnlyEntity>).collect()
    }

    fn get_relation(&self, id: &Uuid) -> Option<Rc<dyn ReadOnlyRelation>> {
        self.get_relation(id).map(|r| r as Rc<dyn ReadOnlyRelation>)
    }

    fn get_all_relations(&self) -> Vec<Rc<dyn ReadOnlyRelation>> {
        self.get_all_relations().into_iter().map(|r| r as Rc<dyn ReadOnlyRelation>).collect()
    }
}
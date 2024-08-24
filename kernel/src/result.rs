use std::fmt;
use std::collections::HashMap;
use uuid::Uuid;
use crate::context::ExecutionContext;
use crate::types::{EntityType, RelationType};
use crate::process::Condition;
use crate::variable::Value;

#[derive(Debug)]
pub enum ExecutionResult {
    UpdateEntityState(Uuid, String, Value),
    DeleteEntityState(Uuid, String),
    CreateEntity(EntityCreationInfo),
    DeleteEntity(Uuid),
    CreateRelation(RelationCreationInfo),
    DeleteRelation(Uuid),
    AddFunction(Uuid, FunctionCreationInfo),
    RemoveFunction(Uuid, String),
    ActivateFunction(Uuid, String),
    DeactivateFunction(Uuid, String),
    UpdateFunctionParameter(Uuid, String, String, Value),
    DeleteFunctionParameter(Uuid, String, String),
    AddProcess(Uuid, String, ProcessCreationInfo),
    RemoveProcess(Uuid, String, String),
    AddCondition(Uuid, String, String, Box<dyn Condition>),
    RemoveCondition(Uuid, String, String),
    AddRelationMetadata(Uuid, String, Value),
    RemoveRelationMetadata(Uuid, String),
}

#[derive(Debug)]
pub struct EntityCreationInfo {
    pub name: String,
    pub entity_type: EntityType,
    pub initial_state: HashMap<String, Value>,
    pub functions: Vec<FunctionCreationInfo>,
    pub relations: Vec<RelationCreationInfo>,
}

#[derive(Debug)]
pub struct FunctionCreationInfo {
    pub name: String,
    pub initial_parameters: HashMap<String, Value>,
    pub processes: Vec<ProcessCreationInfo>,
}

pub struct ProcessCreationInfo {
    pub name: String,
    pub action: Box<dyn Fn(&ExecutionContext) -> Vec<ExecutionResult>>,
    pub condition: Option<Box<dyn Condition>>,
}

impl fmt::Debug for ProcessCreationInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProcessCreationInfo")
            .field("name", &self.name)
            .field("action", &"<function>")
            .field("condition", &self.condition.is_some())
            .finish()
    }
}

#[derive(Debug)]
pub struct RelationCreationInfo {
    pub name: String,
    pub relation_type: RelationType,
    pub target_entity_id: Option<Uuid>,
    pub target_entity_name: Option<String>,
    pub metadata: Option<HashMap<String, Value>>,
}
mod entity;
mod variable;
mod relation;
mod function;
mod process;
mod model;
mod types;
mod context;
mod result;

pub use context::{ReadOnlyEntity, ReadOnlyFunction, ReadOnlyRelation, ReadOnlyModel, ExecutionContext};
pub use result::{ExecutionResult, EntityCreationInfo, RelationCreationInfo, FunctionCreationInfo, ProcessCreationInfo};
pub use entity::Entity;
pub use variable::{Variable, Value};
pub use relation::{Relation, RelationshipDefinition, RelationshipRegistry};
pub use function::Function;
pub use process::{Process, Condition, AlwaysTrueCondition};
pub use model::Model;
pub use types::{EntityType, RelationType};
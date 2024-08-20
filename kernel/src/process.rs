use std::fmt;
use std::rc::Weak;
// use uuid::Uuid;
use crate::Function;
use crate::Entity;

pub struct Process {
    // pub id: Uuid,
    pub name: String,
    pub owner: Weak<Function>,
    condition: Option<Box<dyn Condition>>,
    action: Box<dyn Fn() + 'static>,
}

impl Process {
    pub fn new(
        name: String,
        owner: Weak<Function>,
        action: Box<dyn Fn() + 'static>,
    ) -> Self {
        Process {
            // id: Uuid::new_v4(),
            name,
            owner,
            condition: None,
            action,
        }
    }

    pub fn set_condition(&mut self, condition: Box<dyn Condition>) {
        self.condition = Some(condition);
    }

    pub fn execute(&self) {
        (self.action)();
    }

    pub fn check_condition(&self) -> bool {
        if let Some(condition) = &self.condition {
            if let Some(function) = self.owner.upgrade() {
                if let Some(owner_entity) = function.owner.upgrade() {
                    return condition.is_met(&owner_entity);
                }
            }
        }
        false
    }
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Process")
            // .field("id", &self.id)
            .field("name", &self.name)
            .field("owner", &self.owner)
            .field("condition", &self.condition.is_some())
            .finish()
    }
}

pub trait Condition: fmt::Debug {
    fn is_met(&self, entity: &Entity) -> bool;
}

#[derive(Debug)]
pub struct AlwaysTrueCondition {}

impl Condition for AlwaysTrueCondition {
    fn is_met(&self, _entity: &Entity) -> bool {
        true
    }
}
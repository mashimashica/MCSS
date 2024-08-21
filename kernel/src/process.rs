use std::fmt;
use std::rc::Weak;
use crate::function::Function;
use crate::entity::Entity;

pub struct Process {
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
        if let Some(function) = self.owner.upgrade() {
            if function.is_active() & self.check_condition() {
                (self.action)();
            }
        }
    }

    fn check_condition(&self) -> bool {
        if let Some(condition) = &self.condition {
            if let Some(function) = self.owner.upgrade() {
                if let Some(owner_entity) = function.owner.upgrade() {
                    return condition.is_met(&owner_entity);
                }
            }
        }
        true
    }
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Process")
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
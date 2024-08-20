use std::fmt;
use std::rc::{Rc, Weak};
use uuid::Uuid;
use crate::function::Function;
use crate::entity::Entity;

pub trait Process: fmt::Debug {
    fn id(&self) -> Uuid;
    fn name(&self) -> &str;
    fn owner(&self) -> &Weak<Function>;
    fn execute(&self);
    fn check_condition(&self) -> bool;
}

pub struct SimpleProcess {
    pub id: Uuid,
    pub name: String,
    pub condition: Box<dyn Condition>,
    pub action: Box<dyn Fn()>,
    pub owner: Weak<Function>,
}

impl fmt::Debug for SimpleProcess {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SimpleProcess")
            .field("id", &self.id)
            .field("name", &self.name)
            .field("condition", &self.condition)
            .field("action", &"<function>")
            .field("owner", &self.owner)
            .finish()
    }
}

impl SimpleProcess {
    pub fn new(
        name: String,
        condition: Box<dyn Condition>,
        action: Box<dyn Fn()>,
        owner: Weak<Function>
    ) -> Rc<Self> {
        Rc::new(SimpleProcess {
            id: Uuid::new_v4(),
            name,
            condition,
            action,
            owner,
        })
    }
}

impl Process for SimpleProcess {
    fn id(&self) -> Uuid {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn owner(&self) -> &Weak<Function> {
        &self.owner
    }

    fn execute(&self) {
        (self.action)();
    }

    fn check_condition(&self) -> bool {
        if let Some(function) = self.owner.upgrade() {
            if let Some(owner_entity) = function.owner.upgrade() {
                return self.condition.is_met(&owner_entity);
            }
        }
        false
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
use std::fmt;
use crate::entity::Entity;

pub trait Condition: fmt::Debug {
    fn is_met(&self, entity: &Entity) -> bool;
}

pub trait Process: fmt::Debug {
    fn execute(&self, entity: &Entity);
    fn check_condition(&self, entity: &Entity) -> bool;
}

#[derive(Debug)]
pub struct SimpleProcess {
    pub condition: Box<dyn Condition>,
    pub action: fn(&Entity),
}

impl Process for SimpleProcess {
    fn execute(&self, entity: &Entity) {
        (self.action)(entity);
    }

    fn check_condition(&self, entity: &Entity) -> bool {
        self.condition.is_met(entity)
    }
}

#[derive(Debug)]
pub struct AlwaysTrueCondition {}

impl Condition for AlwaysTrueCondition {
    fn is_met(&self, _entity: &Entity) -> bool {
        true
    }
}
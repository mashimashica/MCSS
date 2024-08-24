use std::fmt;
use std::rc::Weak;
use crate::function::Function;
use crate::context::ExecutionContext;
use crate::result::ExecutionResult;
use std::cell::RefCell;

pub struct Process {
    pub name: String,
    pub owner: Weak<Function>,
    condition: RefCell<Option<Box<dyn Condition>>>,
    action: Box<dyn Fn(&ExecutionContext) -> Vec<ExecutionResult> + 'static>,
}

impl Process {
    pub fn new(
        name: String,
        owner: Weak<Function>,
        action: Box<dyn Fn(&ExecutionContext) -> Vec<ExecutionResult> + 'static>,
    ) -> Self {
        Process {
            name,
            owner,
            condition: RefCell::new(None),
            action,
        }
    }

    pub fn set_condition(&self, condition: Box<dyn Condition>) {
        *self.condition.borrow_mut() = Some(condition);
    }

    pub fn remove_condition(&self) {
        *self.condition.borrow_mut() = None;
    }

    pub fn execute(&self, context: &ExecutionContext) -> Vec<ExecutionResult> {
        if let Some(function) = self.owner.upgrade() {
            if function.is_active() && self.check_condition(context) {
                return (self.action)(context);
            }
        }
        vec![]
    }

    fn check_condition(&self, context: &ExecutionContext) -> bool {
        self.condition.borrow().as_ref().map_or(true, |c| c.is_met(context))
    }
}

impl fmt::Debug for Process {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Process")
            .field("name", &self.name)
            .field("owner", &self.owner)
            .field("condition", &self.condition.borrow().is_some())
            .finish()
    }
}

pub trait Condition: fmt::Debug {
    fn is_met(&self, context: &ExecutionContext) -> bool;
}

#[derive(Debug)]
pub struct AlwaysTrueCondition {}

impl Condition for AlwaysTrueCondition {
    fn is_met(&self, _context: &ExecutionContext) -> bool {
        true
    }
}
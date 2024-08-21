use std::collections::HashMap;
use std::cell::{Cell, RefCell};
use std::rc::{Rc, Weak};
use crate::variable::Variable;
use crate::process::Process;
use crate::entity::Entity;

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub owner: Weak<Entity>,
    pub parameter: RefCell<Variable>,
    pub processes: RefCell<HashMap<String, Rc<Process>>>,
    pub active_status: Cell<bool>
}

impl Function {
    pub fn new(name: String, owner: Weak<Entity>) -> Self {
        Function {
            name,
            owner,
            parameter: RefCell::new(Variable::new()),
            processes: RefCell::new(HashMap::new()),
            active_status: Cell::new(false)
        }
    }

    pub fn get_parameter(&self) -> &RefCell<Variable> {
        &self.parameter
    }
    
    pub fn add_process(&self, process: Rc<Process>) {
        self.processes.borrow_mut().insert(process.name.clone(), process);
    }

    pub fn get_process(&self, name: &str) -> Option<Rc<Process>> {
        self.processes.borrow().get(name).cloned()
    }

    pub fn remove_process(&self, name: &str) -> Option<Rc<Process>> {
        self.processes.borrow_mut().remove(name)
    }

    pub fn is_active(&self) -> bool {
        self.active_status.get()
    }

    pub fn activate(&self) {
        self.active_status.set(true);
    }

    pub fn deactivate(&self) {
        self.active_status.set(false);
    }   
}
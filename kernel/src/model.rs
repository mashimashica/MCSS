use std::fmt;
use std::rc::Rc;
use crate::entity::Entity;

pub struct Model {
    pub entities: Vec<Rc<Rc<Entity>>>,
    pub time: u32,
}

impl Model {
    pub fn new() -> Self {
        Model {
            entities: Vec::new(),
            time: 0,
        }
    }

    pub fn add_entity(&mut self, entity: Rc<Rc<Entity>>) {
        self.entities.push(entity);
    }

    fn proceed(&mut self) {
        for entity in &self.entities {
            let functions = entity.functions.borrow();
            for function in functions.iter() {
                let processes = function.processes.borrow();
                for process in processes.iter() {
                    if process.check_condition() {
                        process.execute();
                    }
                }
            }
        }
        self.time += 1;
    }

    pub fn simulate(&mut self, steps: u32) {
        for _ in 0..steps {
            self.proceed();
        }
    }
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Simulation time: {}", self.time)?;
        for (i, entity) in self.entities.iter().enumerate() {
            writeln!(f, "Entity {}: {:?}", i, entity)?;
        }
        Ok(())
    }
}
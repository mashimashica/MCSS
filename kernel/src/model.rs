use std::fmt;
use crate::entity::Entity;

pub struct Model {
    pub entities: Vec<Entity>,
    pub time: u32,
}

impl Model {
    pub fn new() -> Self {
        Model {
            entities: Vec::new(),
            time: 0,
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }

    fn proceed(&mut self) {
        for entity in &self.entities {
            let mut to_execute = Vec::new();
            
            for function in &entity.functions {
                for process in &function.processes {
                    if process.check_condition(entity) {
                        to_execute.push(process);
                    }
                }
            }
            
            for process in to_execute {
                process.execute(entity);
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
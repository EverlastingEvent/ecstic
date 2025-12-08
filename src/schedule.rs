use crate::system::System;

pub struct Schedule {
    systems: Vec<Box<dyn System>>,
}

impl Schedule {
    pub fn new() -> Self {
        Self {
            systems: Vec::new(),
        }
    }

    pub fn add_system(&mut self, system: Box<dyn System>) {
        self.systems.push(system);
    }

    pub fn run(&mut self, world: &crate::world::World) {
        for system in &mut self.systems {
            system.run(world);
        }
    }
}

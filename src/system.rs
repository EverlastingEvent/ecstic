use crate::world::World;

pub trait System {
    fn run(&mut self, world: &World);
}

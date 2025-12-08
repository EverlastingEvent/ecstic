use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::RwLock;

pub struct Archetype {
    components: RwLock<HashMap<TypeId, Box<dyn Any>>>,
}

impl Archetype {
    pub fn new() -> Self {
        //TODO: реализовать
        unimplemented!()
    }

    pub fn add_component<T: 'static>(&mut self, _component: Vec<T>) {
        //TODO: реализовать
        unimplemented!()
    }

    pub fn get_component_mut<T: 'static>(&mut self) -> Option<&mut Vec<T>> {
        //TODO: реализовать
        unimplemented!()
    }
}

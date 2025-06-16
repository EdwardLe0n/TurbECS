use turbo::prelude::*;

use super::component::ComponentLifecycle;

use super::{transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Entity<T: ComponentLifecycle> {
    pub name: String,
    pub components: Vec<T>,
    pub transform: Transform,
    pub layer: usize,
    pub locat: u32,
}

impl<T: ComponentLifecycle> Entity<T> {

    pub fn new (name : String, vec : Vec<T>) -> Self {

        Self { 
            name: name, components: vec, transform: Transform::new(), layer: 0, locat: rand() 
        }

    }

    pub fn add_component (&mut self, component : T) {

        self.components.push(component);

    }

}

impl<T: ComponentLifecycle> Entity<T> {

    pub fn set_layer (&mut self, some_usize : usize) {
        self.layer = some_usize;
    }

    pub fn get_layer (&self) -> usize{

        return self.layer;

    }

}
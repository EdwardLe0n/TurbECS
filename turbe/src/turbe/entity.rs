use turbo::prelude::*;

use super::component::ComponentLifecycle;

use super::{transform::Transform, size::Size};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Entity<T: ComponentLifecycle> {
    pub name: String,
    pub components: Vec<T>,
    pub transform: Transform,
    pub size: Size,
    pub layer: u8,
    pub locat: u32,
}

impl<T: ComponentLifecycle> Entity<T> {

    pub fn new (name : String, vec : Vec<T>) -> Self {

        Self { 
            name: name, components: vec, transform: Transform::new(), 
            size: Size::new(), layer: 0, locat: rand() 
        }

    }

}
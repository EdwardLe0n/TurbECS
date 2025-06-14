use turbo::prelude::*;

use super::component::ComponentLifecycle;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Entity<T: ComponentLifecycle> {
    pub name: String,
    pub components: Vec<T>,
    pub x: i32,
    pub y: i32,
    pub layer: u8,
    pub locat: u32,
}

impl<T: ComponentLifecycle> Entity<T> {

    pub fn new (name : String, vec : Vec<T>) -> Self {

        Self { name: name, components: vec, x: 0, y: 0, layer: 0, locat: rand() }

    }

}
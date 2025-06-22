use turbo::prelude::*;

// Core directories

use crate::turbe;
use turbe::helpers;

// Necessary imports

use turbe::{component::Component};
use helpers::{transform::Transform, position::Position, size::Size, border::Border};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Template {
    pub some_vals : i32
}

impl Template {

    pub fn new() -> Template {
        return Template{
            some_vals : 0};
    }

}

impl Template {

    pub fn update(&mut self, _transform : &mut Transform) {

        

    }

}
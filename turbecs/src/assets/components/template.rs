use turbo::*;

// Core directories

use crate::turbecs;
use turbecs::helpers;

// Necessary imports

use turbecs::{component_system::component::Component};
use helpers::{transform::Transform, position::Position, size::Size, border::Border};

#[turbo::serialize]
#[derive(PartialEq)]
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
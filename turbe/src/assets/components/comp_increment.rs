use turbo::prelude::*;

// Core directories

use crate::turbe;
use turbe::helpers;

// Necessary imports

use helpers::{transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Template {
    pub count : i32
}

impl Template {

    pub fn new() -> Template {
        return Template{
            count : 0};
    }

}

impl Template {

    pub fn update(&mut self, _transform : &mut Transform) {

        self.count += 1;

    }

    pub fn render_increment(&self, _transform : Transform) {
        text!("{}", self.count;);
    }

}
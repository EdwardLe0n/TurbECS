use turbo::prelude::*;

// Core directories

use crate::{turbe, GameState};
use turbe::helpers;

// Necessary imports

use helpers::{transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct IncrementComponent {
    pub count : i32
}

impl IncrementComponent {

    pub fn new() -> IncrementComponent {
        return IncrementComponent{
            count : 0};
    }

}

impl IncrementComponent {

    pub fn update(&mut self, state : &mut GameState) {

        state.test_var += 1;
        
        // let mut state = GameState::new();

        self.count += 1;

    }

    pub fn render_increment(&self, _transform : Transform) {
        // text!("{}", self.count;);
    }

}
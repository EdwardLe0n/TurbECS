use turbo::prelude::*;

// Core directories

use crate::turbe;
use turbe::helpers;

// Necessary imports

use turbe::{component::Component};
use helpers::{transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct MoveComponent {
    pub move_factor : i32
}

impl MoveComponent {

    pub fn new(_move_factor : i32 ) -> Component {
        return Component::Move(MoveComponent{move_factor : 1});
    }

}

impl MoveComponent {

    pub fn update(&mut self, _transform : &mut Transform) {

        _transform.set_rotation(_transform.get_rotation() + self.move_factor);

    }

}
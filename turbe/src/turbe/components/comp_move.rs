use turbo::prelude::*;

use crate::turbe::transform;

use super::super::{component::Component, transform::Transform, border::Border};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct MoveComponent {
    pub move_factor : i32
}

impl MoveComponent {

    pub fn new(move_factor : i32 ) -> Component {
        return Component::Move(MoveComponent{move_factor : 1});
    }

}

impl MoveComponent {

    pub fn update(&mut self, transform : &mut Transform) {

        transform.set_rotation(transform.get_rotation() + self.move_factor);

    }

}
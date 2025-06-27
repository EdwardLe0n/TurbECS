use turbo::prelude::*;

// Core directories

use crate::turbe;
use turbe::helpers;

// Necessary imports

use turbe::{component_system::component::Component};
use helpers::{transform::Transform, position::Position, size::Size, border::Border};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct CameraComponent {
    pub position : Position,
    pub z : f32
}

impl CameraComponent {

    pub fn new() -> CameraComponent {
        return CameraComponent{
            position : Position::new(), z : 0.0};
    }

    pub fn new_with_xy(_x : i32, _y : i32) -> CameraComponent {
        return CameraComponent {
            position : Position { x: _x, y: _y, rotation: 0 },
            z : 0.0
        };
    }

}

impl CameraComponent {

    pub fn camera_prep(&self) {

        camera::set_xyz(self.position.get_x(), self.position.get_y(), self.z);

    }

}
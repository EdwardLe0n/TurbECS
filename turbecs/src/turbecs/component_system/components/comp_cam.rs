use turbo::*;

// Core directories

use crate::turbecs;
use turbecs::helpers;

// Necessary imports

use helpers::{position::Position};

#[turbo::serialize]
#[derive(PartialEq)]
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
            position : Position::new_with_xy(_x, _y),
            z : 0.0
        };
    }

}

impl CameraComponent {

    pub fn camera_prep(&self) {

        camera::set_xyz(self.position.get_x(), self.position.get_y(), self.z);

    }

}
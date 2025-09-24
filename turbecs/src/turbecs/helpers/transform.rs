use turbo::*;

use super::{position, size};
use position::Position;
use size::Size;

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct Transform {
    pub position : Position,
    pub size : Size
}

impl Transform {

    pub fn new() -> Transform {

        return Transform{position : Position::new(), size : Size::new()};

    }
    
}

// Setters

impl Transform {

    pub fn set_x(&mut self, some_x : i32) {
        self.position.set_x(some_x);
    }

    pub fn set_y(&mut self, some_y : i32) {
        self.position.set_y(some_y);
    }

    pub fn set_rotation(&mut self, some_rotation : i32) {
        self.position.set_rotation(some_rotation);
    }

    pub fn set_width(&mut self, some_width : i32) {
        self.size.set_width(some_width);
    }

    pub fn set_height(&mut self, some_height : i32) {
        self.size.set_height(some_height);
    }

    pub fn set_scale_x(&mut self, some_scale_x : f32) {
        self.size.set_scale_x(some_scale_x);
    }

    pub fn set_scale_y(&mut self, some_scale_y : f32) {
        self.size.set_scale_y(some_scale_y);
    }

    pub fn set_scale(&mut self, some_scale : f32) {
        self.set_scale_x(some_scale);
        self.set_scale_y(some_scale);
    }

}

// Getters

impl Transform {

    pub fn get_x(&self) -> i32 {
        return self.position.get_x();
    }

    pub fn get_y(&self) -> i32 {
        return self.position.get_y();
    }

    pub fn get_rotation(&self) -> i32 {
        return self.position.get_rotation();
    }

    pub fn get_width(&self) -> i32 {
        return self.size.get_width();
    }

    pub fn get_height(&self) -> i32 {
        return self.size.get_height();
    }

    pub fn get_scale_x(&self) -> f32 {
        return self.size.get_scale_x();
    }

    pub fn get_scale_y(&self) -> f32 {
        return self.size.get_scale_y();
    }

    pub fn get_scale(&self) -> f32 {
        return (self.get_scale_x() + self.get_scale_y())/2.0;
    }
    
}

// Nudging

impl Transform {

    pub fn nudge_x(&mut self, some_i32 : i32) {
        self.position.nudge_x(some_i32);
    }

    pub fn nudge_y(&mut self, some_i32 : i32) {
        self.position.nudge_y(some_i32);
    }

}

// Other

impl Transform {
    pub fn get_xy_offset(&self, some_bool : bool) -> (i32, i32) {

        if some_bool {
            return self.position.get_xy_offset(0, 0);
        }

        return self.position.get_xy_offset(self.get_width(), self.get_height());

    }

    pub fn get_x_offset(&self) -> i32 {

        return self.position.get_x_offset(self.get_width(), self.get_height());

    }

    pub fn get_y_offset(&self) -> i32 {

        return self.position.get_y_offset(self.get_width(), self.get_height());

    }
}
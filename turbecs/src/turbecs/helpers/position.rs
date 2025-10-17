use turbo::*;

use super::bound_data;
use bound_data::{BoundData, Horizontal, Vertical};

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct Position {
    pub x : i32,
    pub y : i32,
    pub rotation : i32,
    pub is_bounded : bool,
    pub bound_data : BoundData
}

impl Position {

    pub fn new() -> Position {

        return Position { x: 0, y: 0, rotation: 0, is_bounded : false,
                        bound_data : BoundData::new() }

    }

    pub fn new_with_xy(some_x : i32, some_y : i32) -> Position {

        let mut transform = Position::new();

        transform.set_x(some_x);
        transform.set_y(some_y);

        return transform;

    }

}

impl Position {

    pub fn set_x(&mut self, some_x : i32) {
        self.x = some_x;
    }

    pub fn set_y(&mut self, some_y : i32) {
        self.y = some_y;
    }

    pub fn set_rotation(&mut self, some_rotation : i32) {
        self.rotation = some_rotation;
    }

    pub fn set_bound_status(&mut self, some_bool : bool) {
        self.is_bounded = some_bool;
    }

    pub fn set_ui_status(&mut self, some_bool : bool) {
        self.bound_data.set_ui_status(some_bool);
    }

    pub fn set_horizontal_pref(&mut self, some_pref : Horizontal) {
        self.bound_data.set_horizontal_pref(some_pref);
    }

    pub fn set_vertical_pref(&mut self, some_pref : Vertical) {
        self.bound_data.set_vertical_pref(some_pref);
    }
    
    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return -self.y;
    }

    pub fn get_rotation(&self) -> i32 {
        return self.rotation;
    }

    pub fn get_bound_status(&self) -> bool {
        return self.is_bounded;
    }

    pub fn get_ui_status(&self) -> bool {
        return self.bound_data.get_ui_status();
    }

    pub fn get_horizontal_pref(&self) -> Horizontal {
        return self.bound_data.get_horizontal_pref();
    }

    pub fn get_vertical_pref(&self) -> Vertical {
        return self.bound_data.get_vertical_pref();
    }

}

impl Position {
    
    pub fn nudge_x(&mut self, some_x : i32) {

        self.set_x(self.get_x() + some_x);

    }

    pub fn nudge_y(&mut self, some_y : i32) {

        self.set_y(-self.get_y() + some_y);

    }

    pub fn nudge_rotation(&mut self, some_rotation : i32) {

        self.set_rotation(self.get_rotation() + some_rotation);

    }

}

impl Position {

    pub fn get_adjusted_bounds(&self, bounds : Bounds, other_bounds : Bounds) -> Bounds {

        let mut b1 = bounds.clone();
        let mut b2 = other_bounds.clone();

        match self.get_horizontal_pref() {
            Horizontal::Left => {b1 = b1.anchor_left(&b2);},
            Horizontal::Center => {b1 = b1.anchor_center_x(&b2);},
            Horizontal::Right => {b1 = b1.anchor_right(&b2);}
        }

        match self.get_vertical_pref() {
            Vertical::Top => {b1 = b1.anchor_top(&b2);},
            Vertical::Center => {b1 = b1.anchor_center_y(&b2);},
            Vertical::Bottom => {b1 = b1.anchor_bottom(&b2);}
        }

        return b1;

    }

    pub fn get_xy_offset(&self, width : i32, height : i32) -> (i32, i32) {

        let x = self.get_x_offset(width, height);
        let y = self.get_y_offset(width, height);

        return (x,y);

    }

    pub fn get_x_offset(&self, width : i32, height : i32) -> i32 {

        let mut result = 0;

        if self.get_bound_status() {

            let mut this_bounds = Bounds::new(0, 0, width, height);

            let mut some_other : Bounds;

            if self.get_ui_status() {
                
                some_other = world();

            }
            else {

                some_other = screen();
                
            }

            match self.get_horizontal_pref() {
                Horizontal::Left => {this_bounds = this_bounds.anchor_left(&some_other);},
                Horizontal::Center => {this_bounds = this_bounds.anchor_center_x(&some_other);},
                Horizontal::Right => {this_bounds = this_bounds.anchor_right(&some_other);},
            }

            result += this_bounds.x();

        }

        result += self.get_x();

        return result;

    }

    pub fn get_y_offset(&self, width : i32, height : i32) -> i32 {

        let mut result = 0;

        if self.get_bound_status() {

            let mut this_bounds = Bounds::new(0, 0, width, height);

            let mut some_other = Bounds::new(0, 0, 1, 1);

            if self.get_ui_status() {
                
                some_other = world();

            }
            else {

                // Insert code about making custom bounds here...
                some_other = screen();
                
            }

            match self.get_vertical_pref() {
                Vertical::Top => {this_bounds = this_bounds.anchor_top(&some_other);},
                Vertical::Center => {this_bounds = this_bounds.anchor_center_y(&some_other);},
                Vertical::Bottom => {this_bounds = this_bounds.anchor_bottom(&some_other);},
            }

            result -= this_bounds.y();

        }

        result += self.get_y();

        return result;

    }

}
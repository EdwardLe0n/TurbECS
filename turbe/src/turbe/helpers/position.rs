use turbo::prelude::*;

use super::ui_pref::{Horizonontal, Vertical};

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Position {
    pub x : i32,
    pub y : i32,
    pub rotation : i32,
    pub is_ui : bool,
    pub horizontal : Horizonontal,
    pub vertical : Vertical
}

impl Position {

    pub fn new() -> Position {

        return Position { x: 0, y: 0, rotation: 0, is_ui : false,
                        horizontal : Horizonontal::Center, vertical : Vertical::Center }

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

    pub fn set_ui_status(&mut self, some_bool : bool) {
        self.is_ui = some_bool;
    }

    pub fn set_horizontal_pref(&mut self, some_pref : Horizonontal) {
        self.horizontal = some_pref;
    }

    pub fn set_vertical_pref(&mut self, some_pref : Vertical) {
        self.vertical = some_pref;
    }
    
    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    pub fn get_rotation(&self) -> i32 {
        return self.rotation;
    }

    pub fn get_ui_status(&self) -> bool {
        return self.is_ui;
    }

    pub fn get_horizontal_pref(&self) -> Horizonontal {
        return self.horizontal;
    }

    pub fn get_vertical_pref(&self) -> Vertical {
        return self.vertical;
    }

}

impl Position {
    
    pub fn nudge_x(&mut self, some_x : i32) {

        self.set_x(self.get_x() + some_x);

    }

    pub fn nudge_y(&mut self, some_y : i32) {

        self.set_y(self.get_y() + some_y);

    }

    pub fn nudge_rotation(&mut self, some_rotation : i32) {

        self.set_rotation(self.get_rotation() + some_rotation);

    }

}

impl Position {

    pub fn get_adjusted_bounds(&self, bounds : Bounds, other_bounds : Bounds) -> Bounds {

        let mut b1 = bounds.clone();
        let mut b2 = other_bounds.clone();

        match self.horizontal {
            Horizonontal::Left => {b1 = b1.anchor_left(&b2);},
            Horizonontal::Center => {b1 = b1.anchor_center_x(&b2);},
            Horizonontal::Right => {b1 = b1.anchor_right(&b2);}
        }

        match self.vertical {
            Vertical::Top => {b1 = b1.anchor_top(&b2);},
            Vertical::Center => {b1 = b1.anchor_center_y(&b2);},
            Vertical::Bottom => {b1 = b1.anchor_bottom(&b2);}
        }

        return b1;

    }

}
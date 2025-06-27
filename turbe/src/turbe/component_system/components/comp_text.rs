use turbo::prelude::*;

// Core directories

use crate::turbe;
use turbe::helpers;

// Necessary imports

use helpers::{transform::Transform, position::Position, ui_pref};

use ui_pref::{Horizonontal, Vertical};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TextComponent {
    pub text : String,
    pub position : Position,
    pub color : u32,
    pub font : String
}

impl TextComponent {
    pub fn new(some_str : String) -> TextComponent {

        let some_text = TextComponent {
            text: some_str, position : Position::new(), color : 0xffffffff, font: "medium".to_string()
        };

        return some_text;

    }
}

impl TextComponent {

    pub fn get_text_offset(some_string : &String, some_font : &String) -> Position {

        let mut position = Position::new();

        // Handles the x offset

        match some_font.clone() {
            val if val == "medium".to_string() => {position.set_x(((some_string.len() as i32 / 2) as f32 * 4.5) as i32);},
            _default => {position.set_x((some_string.len() as i32 / 2) * 3);}
        }

        // Handles the y offset

        match some_font {
            _default => {position.set_y(2);}
        }

        return position;

    }

    pub fn render(&self, _transform : Transform) {

        if !self.position.get_ui_status() {

            text!(
                &self.text,
                x = self.position.get_x() + _transform.get_x(),
                y =  -self.position.get_y() -_transform.get_y(),
                color = self.color,
                font = &self.font
            )

        }
        else {

            let canvas_bounds = bounds::canvas();

            let mut bounds = Bounds::with_size(1, 1);

            bounds = self.position.get_adjusted_bounds(bounds, canvas_bounds);

            let some_offset = TextComponent::get_text_offset(&self.text, &self.font);

            let x_off = some_offset.get_x();
            let y_off = some_offset.get_y();

            text!(
                &self.text,
                x = self.position.get_x() + _transform.get_x() + bounds.x() - x_off,
                y = -self.position.get_y() - _transform.get_y() + bounds.y() - y_off,
                color = self.color,
                font = &self.font
            )

        }

    }

}
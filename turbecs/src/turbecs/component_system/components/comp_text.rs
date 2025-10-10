use turbo::*;

// Core directories

use crate::turbecs;
use turbecs::helpers;

// Necessary imports

use helpers::{transform::Transform, position::Position};

#[turbo::serialize]
#[derive(PartialEq)]
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
            val if val == "medium".to_string() => {position.set_x(((some_string.len() as i32 / 2) as f32 * 7.0) as i32);},
            val if val == "large".to_string() => {position.set_x(((some_string.len() as i32 / 2) as f32 * 12.0) as i32);},

            val if val == "TinyUnicodeMedium".to_string() => {position.set_x(((some_string.len() as i32 / 2) as f32 * 12.0) as i32);},
            val if val == "TinyUnicodeLarge".to_string() => {position.set_x(((some_string.len() as i32 / 2) as f32 * 24.0) as i32);},

            _default => {position.set_x((some_string.len() as i32 / 2) * 3);}
        }

        // Handles the y offset

        match some_font.clone() {
            val if val == "medium".to_string() => {position.set_y(4);},
            val if val == "large".to_string() => {position.set_y(6);},

            val if val == "TinyUnicodeMedium".to_string() => {position.set_y(8);},
            val if val == "TinyUnicodeLarge".to_string() => {position.set_y(12);},

            _default => {position.set_y(2);}
        }

        return position;

    }

    pub fn render(&self, _transform : Transform) {

        let some_offset = TextComponent::get_text_offset(&self.text, &self.font);

        let x_off = some_offset.get_x();
        let y_off = some_offset.get_y();

        text!(
            &self.text,
            x = self.position.get_x_offset(0,0) + _transform.get_x_offset() - x_off,
            y =  self.position.get_y_offset(0, 0) + _transform.get_y_offset() + y_off,
            color = self.color,
            font = &self.font,
            fixed = _transform.position.get_ui_status() || self.position.get_ui_status()
        );

    }

}
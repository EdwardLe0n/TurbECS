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

    fn get_preferred_width(some_font : &str) -> f32 {

        match some_font {
            "medium"    => {return 7.0;},
            "large"     => {return 12.0;},
            _default => {},
        }

        return 10.0;

    }

    fn get_preferred_height(some_font : &str) -> f32 {

        match some_font {
            "medium"    => {return 4.0;},
            "large"     => {return 6.0;},
            _default => {},
        }

        return 10.0;

    }

    pub fn get_text_offset(some_string : &String, some_font : &str) -> Position {

        let mut position = Position::new();

        // Handles the x offset

        position.set_x((some_string.len() as f32 / 2.0) * TextComponent::get_preferred_width(some_font));

        // Handles the y offset

        position.set_y(TextComponent::get_preferred_height(some_font));

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
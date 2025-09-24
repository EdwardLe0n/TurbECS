use turbo::*;

// Core directories

use crate::turbecs;
use turbecs::helpers;

// Necessary imports

use helpers::{transform::Transform};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct TextBoxComponent {
    pub text : String,
    pub font : String,
    pub start : f32,
    pub end : f32,
    pub transform : Transform,
    pub color : u32,
}

impl TextBoxComponent {
    
    pub fn new(some_str : String) -> TextBoxComponent {

        let mut some_text = TextBoxComponent {
            text: some_str.clone(),
            font: "medium".to_string(),
            start : 0.0,
            end : some_str.len()as f32,
            transform : Transform::new(),
            color : 0x000000ff, 
        };

        some_text.transform.set_width(100);
        some_text.transform.set_height(100);

        return some_text;

    }

    pub fn replace_str(&mut self, some_string : String) {

        self.text = some_string;
        
        self.start = 0.0;
        self.end = self.text.len() as f32;

    }

}

impl TextBoxComponent {

    pub fn render(&self, _transform : Transform) {

        text_box!(
            &self.text,
            font = &self.font,
            scale = self.transform.get_scale() * _transform.get_scale(),
            start = self.start as usize,
            end = self.end as usize,
            width = self.transform.get_width(),
            height = self.transform.get_height(),
            x = self.transform.position.get_x_offset(0,0) + _transform.get_x_offset() - self.transform.get_width() / 2,
            y =  self.transform.position.get_y_offset(0, 0) + _transform.get_y_offset() + self.transform.get_height() / 2,
            color = self.color,
            align = &self.transform.position.bound_data.get_horizontal_pref().get_string()
        );

    }

}
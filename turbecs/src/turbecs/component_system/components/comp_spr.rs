use turbo::*;

// Core directories

use crate::turbecs;
use turbecs::helpers;

// Necessary imports

use helpers::{transform::Transform, flip::Flip};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct SpriteComponent {
    pub name: String, 
    pub transform: Transform,
    pub color: u32, 
    pub opacity: f32, 
    pub flip_factor: Flip, 
    pub frame: usize
}

impl SpriteComponent {
    
    pub fn new(name : String) -> Self {

        let spr = SpriteComponent {
            name : name,
            transform : Transform::new(),
            color : 0xffffffff,
            opacity : 1.0,
            flip_factor : Flip::new(),
            frame : 0
        };

        return spr;

    }

}

impl SpriteComponent {

    pub fn update_sprite_file(&mut self, some_file : String) {

        self.name = some_file;

    }

}

impl SpriteComponent {

    pub fn render_sprite(&self, transform : Transform) {

        sprite!(
            self.name.as_str(),
            x = self.transform.get_x_offset() + transform.get_x_offset(),
            y = self.transform.get_y_offset() + transform.get_y_offset(),
            w = self.transform.get_width() as f32 * self.transform.get_scale_x() * transform.get_scale_x(),
            h = self.transform.get_height() as f32 * self.transform.get_scale_y() * transform.get_scale_y(),
            color = self.color,
            opacity = self.opacity,
            rotation = self.transform.get_rotation() + transform.get_rotation(),
            scale_x = self.transform.get_scale_x(),
            scale_y = self.transform.get_scale_y(),
            flip_x = self.flip_factor.get_x(),
            flip_y = self.flip_factor.get_y()
        )

    }

}
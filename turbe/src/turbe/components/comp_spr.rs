use turbo::prelude::*;

use crate::turbe::transform;

use super::super::{component::Component, transform::Transform, flip::Flip};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SpriteComponent {
    name: String, 
    transform: Transform,
    color: u32, 
    opacity: f32, 
    flip_factor: Flip, 
    frame: usize
}

impl SpriteComponent {
    
    pub fn new(name : String) -> Component {

        let spr = SpriteComponent {
            name : name,
            transform : Transform::new(),
            color : 0xffffffff,
            opacity : 1.0,
            flip_factor : Flip::new(),
            frame : 0
        };

        return Component::Sprite(spr);

    }

}

impl SpriteComponent {
    
    pub fn render_sprite(&mut self, transform : Transform) {

        sprite!(
            self.name.as_str(),
            x = self.transform.get_x() + transform.get_x(),
            y = self.transform.get_y() + transform.get_y(),
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
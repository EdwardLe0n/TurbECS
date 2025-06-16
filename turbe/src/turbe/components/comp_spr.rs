use turbo::prelude::*;

use super::super::{position::Position, flip::Flip, size::Size};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SpriteComponent {
    name: String, 
    position: Position, 
    size: Size, 
    color: u32, 
    opacity: f32, 
    flip_factor: Flip, 
    frame: usize
}

impl SpriteComponent {
    
    pub fn render_sprite(&mut self) {

        sprite!(
            self.name.as_str(),
            x = self.position.get_x(),
            y = self.position.get_y(),
            w = self.size.get_width(),
            h = self.size.get_height(),
            color = self.color,
            opacity = self.opacity,
            rotation = self.position.get_rotation(),
            scale_x = self.size.get_scale_x(),
            scale_y = self.size.get_scale_y(),
            flip_x = self.flip_factor.get_x(),
            flip_y = self.flip_factor.get_y()
        )

    }

}
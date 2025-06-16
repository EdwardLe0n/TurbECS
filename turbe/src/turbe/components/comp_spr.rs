use turbo::prelude::*;

use super::super::{position::Position, flip::Flip, size::Size};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SpriteComponent {
    name: String, 
    transform: Position, 
    size: Size, 
    color: u32, 
    opacity: f32, 
    flip_factor: Flip, 
    frame: usize
}

pub fn render_sprite(sprite_component : SpriteComponent) {

        sprite!(
            sprite_component.name.as_str(),
            x = sprite_component.transform.get_x(),
            y = sprite_component.transform.get_y(),
            w = sprite_component.size.get_width(),
            h = sprite_component.size.get_height(),
            color = sprite_component.color,
            opacity = sprite_component.opacity,
            rotation = sprite_component.transform.get_rotation(),
            scale_x = sprite_component.size.get_scale_x(),
            scale_y = sprite_component.size.get_scale_y(),
            flip_x = sprite_component.flip_factor.get_x(),
            flip_y = sprite_component.flip_factor.get_y()
        )

}
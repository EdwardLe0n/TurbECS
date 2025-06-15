use turbo::prelude::*;

use super::super::{component::{Component, ComponentLifecycle}, transform::Transform, size::Size, border::Border};

pub fn render_rect(transform : Transform, size : Size, color : u32, border : Border, _x: i32, _y: i32) {
    
    rect!(
        x = transform.get_x(),
        y = transform.get_y(),
        w = size.get_width() as f32 * size.get_scale_x(),
        h = size.get_height(),
        color = color,
        rotation = transform.get_rotation(),
        border_size = border.get_size(),
        border_color = border.get_color(),
        border_radius = border.get_radius(),
    );

}
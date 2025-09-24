use turbo::*;

// Core directories

use crate::turbecs;
use turbecs::helpers;

// Necessary imports

use helpers::{transform::Transform, border::Border};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct RectangleComponent {
    pub transform: Transform, 
    pub color: u32, 
    pub border: Border
}

impl RectangleComponent {
    
    pub fn new_base() -> RectangleComponent {

        return RectangleComponent {
            transform : Transform::new(),
            color : 0xffffffff,
            border : Border::new()
        };

    }

    pub fn new_rect(some_width : i32, some_height : i32, some_color : u32) -> RectangleComponent {

        let mut rectangle = RectangleComponent::new_base();

        rectangle.transform.set_width(some_width);
        rectangle.transform.set_height(some_height);
        rectangle.color = some_color;

        return rectangle;

    }

}

impl RectangleComponent {
    
    pub fn render_rect(&self, transform : Transform) {    
        rect!(
            x = self.transform.get_x_offset() + transform.get_x_offset(),
            y = self.transform.get_y_offset() + transform.get_y_offset(),
            w = self.transform.get_width() as f32 * self.transform.get_scale_x() * transform.get_scale_y(),
            h = self.transform.get_height() as f32 * self.transform.get_scale_y() * transform.get_scale_y(),
            color = self.color,
            rotation = self.transform.get_rotation() + transform.get_rotation(),
            border_size = self.border.get_size(),
            border_color = self.border.get_color(),
            border_radius = self.border.get_radius(),
        );
    }

}
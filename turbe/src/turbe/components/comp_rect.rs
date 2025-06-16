use turbo::prelude::*;

use crate::turbe::transform;

use super::super::{component::Component, transform::Transform, border::Border};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RectangleComponent {
    transform: Transform, 
    color: u32, 
    border: Border
}

impl RectangleComponent {
    
    pub fn new_base() -> RectangleComponent {

        return RectangleComponent {
            transform : Transform::new(),
            color : 0xffffffff,
            border : Border::new()
        };

    }

    pub fn new_rect(some_width : i32, some_height : i32, some_color : u32) -> Component {

        let mut rectangle = RectangleComponent::new_base();

        rectangle.transform.set_width(some_width);
        rectangle.transform.set_height(some_height);
        rectangle.color = some_color;

        return Component::Rectangle(rectangle);

    }

}

impl RectangleComponent {
    
    pub fn render_rect(&mut self, transform : Transform) {    
        rect!(
            x = self.transform.get_x() + transform.get_x(),
            y = self.transform.get_y() + transform.get_y(),
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
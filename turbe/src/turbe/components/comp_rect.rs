use turbo::prelude::*;

use super::super::{component::Component, position::Position, size::Size, border::Border};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RectangleComponent {
    position: Position, 
    size: Size, 
    color: u32, 
    border: Border
}

impl RectangleComponent {
    
    pub fn new_base() -> RectangleComponent {

        return RectangleComponent {
            position : Position::new(), 
            size : Size::new(),
            color : 0xffffffff,
            border : Border::new()
        };

    }

    pub fn new_rect(some_width : i32, some_height : i32, some_color : u32) -> Component {

        let mut rectangle = RectangleComponent::new_base();

        rectangle.size.set_width(some_width);
        rectangle.size.set_height(some_height);
        rectangle.color = some_color;

        return Component::Rectangle(rectangle);

    }

}

impl RectangleComponent {
    
    pub fn render_rect(&mut self) {    
        rect!(
            x = self.position.get_x(),
            y = self.position.get_y(),
            w = self.size.get_width() as f32 * self.size.get_scale_x(),
            h = self.size.get_height(),
            color = self.color,
            rotation = self.position.get_rotation(),
            border_size = self.border.get_size(),
            border_color = self.border.get_color(),
            border_radius = self.border.get_radius(),
        );
    }

}
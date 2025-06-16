use turbo::prelude::*;

use super::super::{component::Component, position::Position, size::Size, border::Border};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct RectangleComponent {
    transform: Position, 
    size: Size, 
    color: u32, 
    border: Border
}

pub fn new_base() -> RectangleComponent {

    return RectangleComponent {
        transform : Position::new(), 
        size : Size::new(),
        color : 0xffffffff,
        border : Border::new()
    };

}

pub fn new_rect(some_width : i32, some_height : i32, some_color : u32) -> Component {

    let mut rectangle = new_base();

    rectangle.size.set_width(some_width);
    rectangle.size.set_height(some_height);
    rectangle.color = some_color;

    return Component::Rectangle(rectangle);

}

pub fn render_rect(rectangle_component : RectangleComponent) {
    
    rect!(
        x = rectangle_component.transform.get_x(),
        y = rectangle_component.transform.get_y(),
        w = rectangle_component.size.get_width() as f32 * rectangle_component.size.get_scale_x(),
        h = rectangle_component.size.get_height(),
        color = rectangle_component.color,
        rotation = rectangle_component.transform.get_rotation(),
        border_size = rectangle_component.border.get_size(),
        border_color = rectangle_component.border.get_color(),
        border_radius = rectangle_component.border.get_radius(),
    );

}
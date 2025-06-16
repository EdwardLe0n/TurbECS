use crate::turbe;
use turbe::{entity::Entity, component::Component, position::Position, size::Size, border::Border};

use turbe::components::{comp_rect::RectangleComponent};

pub fn new_rect() -> Entity<Component> {
    
    return Entity::new("Some rect".to_string(), 
        vec![
            RectangleComponent::new_rect(10, 10, 0x12345ff)
        ]
    );

}
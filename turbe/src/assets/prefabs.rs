use crate::turbe;
use turbe::{entity::Entity, component::Component, transform::Transform, size::Size, border::Border};

use turbe::components::{comp_rect};

pub fn new_rect() -> Entity<Component> {
    
    return Entity::new("Some rect".to_string(), 
        vec![
            comp_rect::new_rect(10, 10, 0x12345ff)
        ]
    );

}
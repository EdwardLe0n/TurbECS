use crate::turbe;
use turbe::{entity::Entity, component::{Component, ComponentLifecycle}, transform::Transform, size::Size};

pub fn new_rect() -> Entity<Component> {
    

    return Entity::new("Some rect".to_string(), 
    vec![Component::Rectangle { size : Size::new_with_wh(10, 10) , transform: Transform::new()}]);

}
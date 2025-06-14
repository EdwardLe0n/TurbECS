use crate::turbe;
use turbe::{entity::Entity, component::{Component, ComponentLifecycle}};

pub fn new_rect() -> Entity<Component> {

    return Entity::new("Some rect".to_string(), 
    vec![Component::Rectangle { width: 10 }]);

}
use crate::turbe;
use turbe::{entity::Entity, component::{Component, ComponentLifecycle}, transform::Transform, size::Size, border::Border};

pub fn new_rect() -> Entity<Component> {
    

    return Entity::new("Some rect".to_string(), 
    vec![Component::Rectangle { 
        transform: Transform::new(), size : Size::new_with_wh(10, 10), color : 0x123456ff, border : Border::new()
    }]);

}
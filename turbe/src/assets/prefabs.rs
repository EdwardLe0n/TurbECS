use crate::turbe;
use turbe::{entity::Entity, component::Component, border::Border};

use turbe::components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent};

pub fn new_rect() -> Entity<Component> {
    
    return Entity::new("Some rect".to_string(), 
        vec![
            RectangleComponent::new_rect(10, 10, 0x12345ff)
        ]
    );

}

pub fn new_spr() -> Entity<Component> {

    let mut ent = Entity::new("some image".to_string(), vec![]);

    ent.transform.set_x(100);

    ent.add_component(SpriteComponent::new("smile".to_string()));

    return ent;

}
use crate::turbe;
use turbe::{entity::Entity, component::{Component, ComponentLifecycle}, border::Border};

use turbe::components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent, comp_text::TextComponent, comp_move::MoveComponent};

pub fn new_text () -> Entity<Component> {

    let mut ent = Entity::new("some text".to_string(), vec![]);
    ent.set_layer(10);

    ent.add_component(TextComponent::new("some text".to_string()));

    return ent;

}

pub fn new_rect () -> Entity<Component> {
    
    let mut ent = Entity::new("some rect".to_string(), vec![]);

    ent.add_component(RectangleComponent::new_rect(10, 25, 0x123456ff));
    ent.add_component(MoveComponent::new(-1));

    ent.set_layer(3);

    return ent;

}

pub fn new_spr () -> Entity<Component> {

    let mut ent = Entity::new("some image".to_string(), vec![]);

    ent.transform.set_x(100);

    ent.add_component(SpriteComponent::new("smile".to_string()));

    return ent;

}
use crate::turbe;
use turbe::helpers;

use turbe::{entity::Entity, component_system};
use component_system::component::{Component, ComponentData};

// Standard Components
use component_system::components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent, comp_text::TextComponent, comp_butn::ButtonComponent};
use component_system::components::buttons::button_types::ButtonTypes;

// User defined components
use crate::assets;
use assets::components::{comp_move::MoveComponent, comp_increment::IncrementComponent};

pub fn new_text () -> Entity<Component> {

    let mut ent = Entity::new("some text".to_string(), vec![]);
    ent.set_layer(10);

    ent.add_component(TextComponent::new("some text".to_string()));

    return ent;

}

pub fn new_rect () -> Entity<Component> {
    
    let mut ent = Entity::new("some rect".to_string(), vec![]);

    ent.add_component(Component::new(ComponentData::Rectangle(RectangleComponent::new_rect(10, 25, 0x123456ff))));
    ent.add_component(Component::new(ComponentData::Move(MoveComponent::new(-1))));

    ent.add_component(Component::new(ComponentData::Increment(IncrementComponent::new())));

    ent.set_layer(3);

    return ent;

}

pub fn new_spr () -> Entity<Component> {

    let mut ent = Entity::new("some image".to_string(), vec![]);

    ent.transform.set_x(100);

    ent.add_component(SpriteComponent::new("smile".to_string()));

    return ent;

}

pub fn new_button () -> Entity<Component> {

    let mut ent = Entity::new("some button".to_string(), vec![]);

    let mut button = ButtonComponent::new();

    button.transform.set_width(30);
    button.transform.set_height(30);
    button.button_type = ButtonTypes::Test;

    ent.add_component(Component::new(ComponentData::Button(button)));

    return ent;

}
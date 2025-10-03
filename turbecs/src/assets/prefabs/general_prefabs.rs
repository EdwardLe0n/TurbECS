use turbo::*;

use crate::turbecs;

use turbecs::{entity::Entity, component_system};
use component_system::component::{Component, ComponentData};

// Standard Components
use component_system::components::{comp_text::TextComponent, comp_text_box::TextBoxComponent, comp_butn::ButtonComponent};
use component_system::components::buttons::button_types::ButtonTypes;

use component_system::components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent};

use turbecs::helpers::{bound_data};

// User defined components
use crate::assets;

use assets::components::{misc_components};

use misc_components::{comp_resizer::ResizerComponent, comp_textbox_resizer::TextBoxResizerComponent};
use misc_components::{comp_screen_manager::ScreenManagerComponent, comp_text_box_filler::TextBoxFillerComponent};
use misc_components::{comp_fade::FadeComponent};

pub fn new_screen_manager() -> Entity {

    let mut ent = Entity::new_base("Screen Manager".to_string());

    ent.add_component(
        Component::new(
            ComponentData::ScreenManager(
                ScreenManagerComponent::new()
            )
        )
    );

    return ent;

}

pub fn new_swipe_right() -> Entity {

    let mut ent = Entity::new_base("Swipe right button".to_string());

    ent.set_layer(1);

    ent.transform.nudge_x((screen().w() as i32 / 9) * 4);

    let mut the_button = ButtonComponent::new();

    the_button.button_type = ButtonTypes::SwipeRight;

    the_button.transform.set_width(20);
    the_button.transform.set_height(40);

    the_button.color = 0x00000000;

    ent.add_component(
        Component::new(
            ComponentData::Button(
                the_button
            )
        )
    );

    let mut button_spr = SpriteComponent::new("buttons/swipe_right_butn".to_string());

    button_spr.transform.set_width(20);
    button_spr.transform.set_height(40);

    button_spr.transform.nudge_x(-10);
    button_spr.transform.nudge_y(20);

    ent.add_component(
        Component::new(
            ComponentData::Sprite(
                button_spr
            )
        )
    );

    return ent;

}

pub fn new_swipe_left() -> Entity {

    let mut ent = Entity::new_base("Swipe left button".to_string());

    ent.set_layer(1);

    ent.transform.nudge_x((screen().w() as i32 / 9) * -4);

    let mut the_button = ButtonComponent::new();

    the_button.button_type = ButtonTypes::SwipeLeft;

    the_button.transform.set_width(20);
    the_button.transform.set_height(40);

    the_button.color = 0x00000000;

    ent.add_component(
        Component::new(
            ComponentData::Button(
                the_button
            )
        )
    );

    let mut button_spr = SpriteComponent::new("buttons/swipe_left_butn".to_string());

    button_spr.transform.set_width(20);
    button_spr.transform.set_height(40);

    button_spr.transform.nudge_x(-10);
    button_spr.transform.nudge_y(20);

    ent.add_component(
        Component::new(
            ComponentData::Sprite(
                button_spr
            )
        )
    );

    return ent;

}

pub fn new_title () -> Entity {

    let mut ent = Entity::new_base("Title".to_string());
    ent.set_layer(10);

    ent.transform.set_y(screen().h() as i32 / 6);

    let mut text_box = TextBoxComponent::new("TurbECS".to_string());

    text_box.font = "TinyUnicodeLarge".to_string();
    text_box.color = 0x000000ff;

    text_box.transform.set_width(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_x() * 4);
    text_box.transform.set_height(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_y() as i32 * -2);
    // text_box.transform.set_scale(1.2);

    text_box.transform.position.set_horizontal_pref(bound_data::Horizonontal::Center);

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    return ent;

}

pub fn new_to_misc() -> Entity {

    let mut ent = Entity::new_base("Misc".to_string());

    ent.transform.position.nudge_y( -200);

    ent.transform.position.set_vertical_pref(bound_data::Vertical::Bottom);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    button.button_type = ButtonTypes::Misc;

    ent.add_component(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Misc".to_string());

    text_box.font = "large".to_string();
    text_box.color = 0xffffffff;

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    ent.add_component(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return ent;

}
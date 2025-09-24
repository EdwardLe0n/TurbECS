use turbo::*;

use crate::turbe;

use turbe::{entity::Entity, component_system};
use component_system::component::{Component, ComponentData};

// Standard Components
use component_system::components::{comp_text::TextComponent, comp_text_box::TextBoxComponent, comp_butn::ButtonComponent};
use component_system::components::buttons::button_types::ButtonTypes;

use component_system::components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent};

use turbe::helpers::{bound_data};

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

    let mut text_box = TextBoxComponent::new("Wumble".to_string());

    text_box.font = "TinyUnicodeLarge".to_string();
    text_box.color = 0x000000ff;

    text_box.transform.set_width(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_x() * 4);
    text_box.transform.set_height(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_y() as i32 * -2);
    // text_box.transform.set_scale(1.2);

    text_box.transform.position.set_horizontal_pref(bound_data::Horizonontal::Center);

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    return ent;

}

pub fn new_ver_notice() -> Entity {

    let mut ent = Entity::new_base("Version number~~~".to_string());
    ent.set_layer(10);

    ent.transform.set_y(screen().h() as i32 * 2 / -5);

    let mut text_box = TextBoxComponent::new("Version 0.1.6".to_string());

    text_box.transform.set_width(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_x() * 4);
    text_box.transform.set_height(TextComponent::get_text_offset(&text_box.text, &text_box.font).get_y() as i32 * -2);
    text_box.transform.set_scale(1.5);

    text_box.font = "TinyUnicodeSmall".to_string();
    text_box.color = 0x000000ff;

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    return ent;

}

pub fn new_to_play() -> Entity {

    let mut ent = Entity::new_base("Play Button".to_string());

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.button_type = ButtonTypes::ToIntro;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    ent.add_component(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Play".to_string());

    text_box.font = "TinyUnicodeMedium".to_string();
    text_box.color = 0xffffffff;

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    ent.add_component(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return ent;

}

pub fn new_to_options() -> Entity {

    let mut ent = Entity::new_base("Options".to_string());

    ent.transform.position.nudge_y(-30);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    button.button_type = ButtonTypes::SwipeRight;

    ent.add_component(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Change Profile".to_string());

    text_box.font = "TinyUnicodeMedium".to_string();
    text_box.color = 0xffffffff;

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    ent.add_component(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return ent;

}

pub fn new_to_live_feed() -> Entity {

    let mut ent = Entity::new_base("Live Feed Button".to_string());

    ent.transform.position.nudge_y( -200);

    ent.transform.position.set_vertical_pref(bound_data::Vertical::Bottom);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    button.button_type = ButtonTypes::ToLiveFeed;

    ent.add_component(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("To the Live Feed!".to_string());

    text_box.font = "large".to_string();
    text_box.color = 0xffffffff;

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    ent.add_component(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

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

pub fn new_main_quit() -> Entity {

    let mut ent = Entity::new_base("Quit".to_string());

    ent.transform.position.nudge_y(-65);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    button.button_type = ButtonTypes::Quit;

    ent.add_component(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Quit".to_string());

    text_box.font = "TinyUnicodeMedium".to_string();
    text_box.color = 0xffffffff;

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    ent.add_component(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return ent;

}

// The reset button

pub fn new_reset() -> Entity {

    let mut ent = Entity::new_base("Restart".to_string());

    ent.set_layer(20);

    ent.transform.position.nudge_y((screen().h() as i32 / -2) + 20);
    ent.transform.position.nudge_x((screen().w() as i32 / -2) + 20);

    let mut button = ButtonComponent::new();

    button.color = 0x11111111;

    button.transform.set_width(40);
    button.transform.set_height(40);

    button.button_type = ButtonTypes::Restart;

    ent.add_component(
        Component::new(
            ComponentData::Button(
                button
            )
        )
    );

    return ent;

}

// Intro scene stuff

pub fn new_intro_to_play() -> Entity {

    let mut ent = Entity::new_base("Play Button".to_string());

    ent.transform.nudge_y(-(2 * screen().h() as i32) / 5);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.button_type = ButtonTypes::Play;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    ent.add_component(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Start!".to_string());

    text_box.font = "TinyUnicodeMedium".to_string();
    text_box.color = 0xffffffff;

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    ent.add_component(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return ent;

}

// Ready scene stuff

pub fn new_to_ready() -> Entity {

    let mut ent = Entity::new_base("Ready to start?".to_string());

    ent.transform.position.nudge_y(3 * (screen().h() / 8) as i32);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xaaaaaaff);

    button.button_type = ButtonTypes::ToReady;

    ent.add_component(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Confirm Deck Here!".to_string());

    text_box.font = "TinyUnicodeMedium".to_string();
    text_box.color = 0xffffffff;

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    ent.add_component(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return ent;

}

// Will finish later
pub fn new_battle_notif() -> Entity { 

    let mut ent = Entity::new_base("Battle Notif".to_string());

    ent.transform.nudge_y((2 * screen().h() as i32) / 5);

    ent.set_layer(8);

    let some_width = 300;
    let some_height = 125;

    let mut rect = RectangleComponent::new_rect(some_width, some_height, 0x000000ff);

    rect.border.set_radius(5);
    
    rect.transform.nudge_x(-some_width / 2);
    rect.transform.nudge_y(some_height / 4);

    ent.add_component(Component::new(ComponentData::Rectangle(rect)));

    let mut text_box = TextBoxComponent::new("Time to Wumble!".to_string());

    text_box.transform.set_width(some_width);
    text_box.transform.set_height(some_height);
    
    text_box.transform.set_y(some_height / 2);

    text_box.color = 0xffffffff;
    text_box.font = "TinyUnicodeLarge".to_string();

    ent.add_component(Component::new(ComponentData::TextBox(text_box)));

    let mut some_button = ButtonComponent::new();

    some_button.transform.set_width(some_width * 2 / 3);
    some_button.transform.set_height(some_height / 4);

    some_button.transform.nudge_y(-some_height / 2);

    some_button.button_type = ButtonTypes::ToBattle;
    some_button.color = 0x000000ff;

    some_button.border.set_color(0xffffffff);
    some_button.border.set_radius(3);
    some_button.border.set_size(2);

    ent.add_component(Component::new(ComponentData::Button(some_button)));

    let mut some_other_text_box = TextBoxComponent::new("It's go time!".to_string());

    some_other_text_box.color = 0xffffffff;
    some_other_text_box.font = "TinyUnicodeMedium".to_string();

    let offset = TextComponent::get_text_offset(&some_other_text_box.text, &some_other_text_box.font);
    
    some_other_text_box.transform.set_width(offset.x * 2 + 2);
    some_other_text_box.transform.set_height(offset.y * 2 + 2);

    some_other_text_box.transform.nudge_y(5 * (-some_height / 12));

    ent.add_component(Component::new(ComponentData::TextBox(some_other_text_box)));

    return ent;
    
}

pub fn new_interaction_info() -> Entity {

    let mut ent = Entity::new_base("Interaction mult info!".to_string());

    ent.set_layer(3);

    let mut some_text_box = TextBoxComponent::new("".to_string());

    some_text_box.font = "TinyUnicodeMedium".to_string();

    some_text_box.transform.set_height(screen().h() as i32 * 4 / 5);
    some_text_box.transform.set_width(screen().w() as i32 * 4 / 5);

    some_text_box.transform.nudge_y(screen().h() as i32 * 4 / 5);

    ent.add_component(
        Component::new(
            ComponentData::TextBox(
                some_text_box
            )
        )
    );

    return ent;

}
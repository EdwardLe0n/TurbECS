// Core directories

use turbo::*;

use crate::{turbecs::{self, component_system::components::buttons::button_types::ButtonTypes}, GameState};

use turbecs::entity::Entity;
use turbecs::component_system;
use component_system::{component::ComponentData, component_types::ComponentTypes, components};
use components::{comp_text::TextComponent};
use turbecs::helpers::position::Position;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct TextBoxResizerComponent {
    pub w_buffer : u32,
    pub h_buffer : u32
}

impl TextBoxResizerComponent {

    pub fn new() -> TextBoxResizerComponent {
        return TextBoxResizerComponent { w_buffer: 0, h_buffer: 0 };
    }

    pub fn new_with_buffers(_w : u32, _h : u32) -> TextBoxResizerComponent {
        
        let mut resize = TextBoxResizerComponent::new();

        resize.w_buffer = _w;
        resize.h_buffer = _h;
        
        return resize;
    }


}

impl TextBoxResizerComponent {

    pub fn on_awake (&mut self, ent : &mut Entity, state : &mut GameState) {

        let locat_text = ent.find_component_in_state(ComponentTypes::TextBox, state);
        let locat_button = ent.find_component_in_state(ComponentTypes::Button, state);

        if !locat_text.0 {
            log!("No text box!");
            return;
        }

        if !locat_button.0 {
            log!("No button!");
            return;
        }

        let mut size = Position::new();

        if let ComponentData::TextBox(text_box) = &mut state.component_manager.components[locat_text.1].component_data {
            size = TextComponent::get_text_offset(&text_box.text, &text_box.font);

            if text_box.text.len() <= 3 as usize {
                size.set_x(7 * text_box.text.len() as i32);
            }

            text_box.transform.set_width(size.get_x() * 3);
            text_box.transform.set_height(size.get_y() * -2);

            text_box.transform.nudge_y(-size.get_y() * 2 - self.h_buffer as i32);

            if text_box.transform.get_width() > screen().w() as i32 * 8 / 10 {
                text_box.transform.set_width(screen().w() as i32 * 8 / 10);
            }

        }

        if let ComponentData::Button(button) = &mut state.component_manager.components[locat_button.1].component_data {

            log!("updated button");

            button.transform.set_width(size.get_x() * 2 + 3 + self.w_buffer as i32 * 2);
            button.transform.set_height(-size.get_y() * 3 + self.h_buffer as i32 * 2);
            button.transform.position.nudge_x(-1);

            if button.transform.get_width() > screen().w() as i32 * 8 / 10 {
                button.transform.set_width(screen().w() as i32 * 8 / 10);
            }

        }

    }

}
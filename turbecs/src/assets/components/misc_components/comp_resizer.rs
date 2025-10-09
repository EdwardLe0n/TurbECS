// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::entity::Entity;
use turbecs::component_system;
use component_system::{component::ComponentData, component_types::ComponentTypes, components};
use components::{comp_text::TextComponent};
use turbecs::helpers::position::Position;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct ResizerComponent {
    pub w_buffer : u32,
    pub h_buffer : u32
}

impl ResizerComponent {

    pub fn new() -> ResizerComponent {
        return ResizerComponent { w_buffer: 0, h_buffer: 0 };
    }

    pub fn new_with_buffers(_w : u32, _h : u32) -> ResizerComponent {
        
        let mut resize = ResizerComponent::new();

        resize.w_buffer = _w;
        resize.h_buffer = _h;
        
        return resize;
    }


}

impl ResizerComponent {

    pub fn on_awake (&mut self, ent : &mut Entity, state : &mut GameState) {

        let locat_text = ent.find_component_in_state(ComponentTypes::Text, state);
        let locat_button = ent.find_component_in_state(ComponentTypes::Button, state);

        if !locat_text.0 {
            log!("No text!");
            return;
        }

        if !locat_button.0 {
            log!("No button!");
            return;
        }

        let mut size = Position::new();

        if let ComponentData::Text(text) = &mut state.component_manager.components[locat_text.1].component_data {
            size = TextComponent::get_text_offset(&text.text, &text.font);

        }

        if let ComponentData::Button(button) = &mut state.component_manager.components[locat_button.1].component_data {

            button.transform.set_width(size.get_x() * 2 + 3 + self.w_buffer as i32 * 2);
            button.transform.set_height(-size.get_y() * 3 + self.h_buffer as i32 * 2);
            button.transform.position.nudge_x(-1);

        }

    }

}
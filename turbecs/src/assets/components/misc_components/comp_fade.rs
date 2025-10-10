// Core directories

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::entity::Entity;
use turbecs::component_system;

use component_system::{component::ComponentData, component_types::ComponentTypes};

const ALIVE_TIME : u32 = 180;

#[turbo::serialize]
#[derive(PartialEq)]
pub struct FadeComponent {
    pub frame_count : u32,
    pub da_tween : Tween<u32>
}

impl FadeComponent {
    pub fn new() -> Self {
        return Self{
            frame_count : 0, 
            da_tween : Tween::new(0x000000ff)
                .set(0x00000000)
                .ease(Easing::EaseInOutQuad)
                .duration(ALIVE_TIME as usize)
        };
    }
}

impl FadeComponent {

    pub fn update(&mut self, ent : &mut Entity, state : &mut GameState) {

        self.frame_count += 1;

        let t_box_locat = ent.find_component_in_state(ComponentTypes::TextBox, state);

        if t_box_locat.0 {

            if let ComponentData::TextBox(some_t_box_comp) = &mut state.component_manager.components[t_box_locat.1].component_data {

                some_t_box_comp.color &= 0xffffff00;
                
                some_t_box_comp.color += self.da_tween.get() as u32;

            }

        }

        let rect_locat = ent.find_component_in_state(ComponentTypes::Rectangle, state);

        if rect_locat.0 {

            if let ComponentData::Rectangle(rect_comp) = &mut state.component_manager.components[rect_locat.1].component_data {

                rect_comp.color &= 0xffffff00;
                rect_comp.color += self.da_tween.get() as u32;

            }

        }

        if self.frame_count >= ALIVE_TIME {
            state.entity_manager.lifetime_data.new_destroy.push_back(ent.locat);
        }

    }

}
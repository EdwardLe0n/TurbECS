use crate::{turbecs, GameState};
use turbecs::entity::Entity;
use turbecs::component_system::{component::ComponentData, component_types::ComponentTypes};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct TextBoxFillerComponent {
    pub found_locat : bool,
    pub locat : usize,
    pub complete : bool,
    pub speed : f32
}

impl TextBoxFillerComponent {

    pub fn new(some_speed : f32) -> Self {
        return Self{found_locat : false, locat : 0, complete : false, speed : some_speed};
    }

}

impl TextBoxFillerComponent {

    pub fn on_awake(&mut self, ent : &mut Entity, state : &mut GameState) {

        let some_locat = ent.find_component_in_state(ComponentTypes::TextBox, state);

        if some_locat.0 {

            self.found_locat = true;
            self.locat = some_locat.1;

            if let ComponentData::TextBox(some_text_box) = &mut state.component_manager.components[self.locat].component_data {
            
                some_text_box.end = 0.0;

            }

        }

    }

    pub fn update(&mut self, ent : &mut Entity, state : &mut GameState) {

        if !self.found_locat {
            return;
        }

        if self.complete {
            return;
        }

        if let ComponentData::TextBox(some_text_box) = &mut state.component_manager.components[self.locat].component_data {
            
            some_text_box.end += self.speed;

            if some_text_box.text.len() <= some_text_box.end as usize {
                some_text_box.end = some_text_box.text.len() as f32;
                self.complete = true;
            }

        }

    }

}
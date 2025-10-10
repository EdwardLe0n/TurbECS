use crate::{turbecs, GameState};
use turbecs::{entity::Entity,component_system::component};
use component::{Component, ComponentData};

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct HasX {
    pub has_awake : bool,
    pub has_start : bool,
    pub has_update : bool,
    pub has_render : bool,
    pub has_destroy : bool,
}

// Initializers

impl HasX {

    // Default constructor that initializes the 'has' functionality of all components to 0
    pub fn new() -> Self {
        return Self{
            has_awake : false,
            has_start : false,
            has_update : false,
            has_render : false,
            has_destroy : false
        };
    }

}

impl Component {
    
    // Initializes a component to let it know that it has certain lifetime functions that it needs to worry about
    pub fn init_has_x(&mut self) {

        self.init_awake();
        self.init_start();
        self.init_update();
        self.init_render();
        self.init_destroy();

    }

    fn init_awake(&mut self) {

        match &self.component_data {
            // Extra functionality
            ComponentData::TextBoxResizer(_)=> {self.has.has_awake = true;},

            _default => {}
        }

    }

    fn init_start(&mut self) {

        match &self.component_data {
            _default => {}
        }

    }

    fn init_update(&mut self) {

        match &self.component_data {

            // Core functionality
            ComponentData::Button(_) => {self.has.has_update = true;},
            
            _default => {}
        }

    }

    fn init_render(&mut self) {

        match &self.component_data {

            // Core functionality
            ComponentData::Button(_)        => {self.has.has_render = true;},
            ComponentData::Text(_)          => {self.has.has_render = true;},
            ComponentData::TextBox(_)       => {self.has.has_render = true;},
            ComponentData::Sprite(_)        => {self.has.has_render = true;},
            ComponentData::Rectangle(_)     => {self.has.has_render = true;},

            // Extra functionality
            
            _default => {}
        }

    }

    fn init_destroy(&mut self) {

        match &self.component_data {
            _default => {}
        }

    }

}

impl Entity {

    // Loops through all the child components to check if they need to do something
    // within a given lifetime function
    pub fn init_has_x(&mut self, state : &mut GameState) {

        let len = self.comp_locats.len();

        for i in 0..len {

            if state.component_manager.components[self.comp_locats[i]].has.has_awake {
                self.has.has_awake = true;
            }

            if state.component_manager.components[self.comp_locats[i]].has.has_start {
                self.has.has_start = true;
            }

            if state.component_manager.components[self.comp_locats[i]].has.has_update {
                self.has.has_update = true;
            }

            if state.component_manager.components[self.comp_locats[i]].has.has_destroy {
                self.has.has_destroy = true;
            }

            if state.component_manager.components[self.comp_locats[i]].has.has_render {
                self.has.has_render = true;
            }

        }

    }

}
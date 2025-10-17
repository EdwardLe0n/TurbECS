use turbo::*;

// Core directories

use crate::turbecs;
use crate::GameState;
use turbecs::helpers;

use crate::pointer;

// Necessary imports

use turbecs::entity::Entity;
use turbecs::component_system;

use component_system::components::buttons::button_types::ButtonTypes;
use helpers::{transform::Transform, border::Border, substates::SubStates};

// Any button func files go here!

use component_system::components::buttons;

use buttons::{scene_loader_buttons, misc_buttons};

// Scene loaders <3

use scene_loader_buttons::{to_misc, to_title};

// Custom states to deal with the three main instances

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ButtonStates {
    None,
    Hover,
    Press
}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct ButtonComponent {
    pub transform : Transform,
    pub border : Border,
    pub color : u32, 
    pub button_type : ButtonTypes,
    pub state : ButtonStates,
    pub sub_state : SubStates,
    pub hov_count : u32,
    pub push_count : u32
}

impl ButtonComponent {

    pub fn new() -> ButtonComponent {
        return ButtonComponent{
            transform : Transform::new(), border : Border::new(), 
            color : 0xffffffff, button_type : ButtonTypes::Default,
            state : ButtonStates::None, sub_state : SubStates::None,
            hov_count : 0, push_count : 0
        };
    }

}

impl ButtonComponent {

    pub fn on_awake (&mut self, _ent : &mut Entity, _state : &mut GameState) {

        match &mut self.button_type {
            _default => {}
        }

    }

    pub fn on_start (&mut self, _ent : &mut Entity, _state : &mut GameState) {

        match &mut self.button_type {
            _default => {}
        }
        
    }

    pub fn update(&mut self, _ent : &mut Entity, _state : &mut GameState) {

        if !_state.can_interact {

            if !self.button_type.can_still_interact(_state) {

                    if self.state != ButtonStates::None {
                    self.state = ButtonStates::None;
                }

                return;

            }

        }

        let mut some_bounds = Bounds::with_size(self.transform.get_width() as f32 * self.transform.get_scale_x() * _ent.transform.get_scale_x(), 
                                            self.transform.get_height() as f32 * self.transform.get_scale_y() * _ent.transform.get_scale_y());

        let offset = self.transform.get_xy_offset(false);

        some_bounds = some_bounds.position_xy((offset.0 + _ent.transform.get_x_offset(), offset.1 + _ent.transform.get_y_offset()));
        some_bounds = some_bounds.translate(-(some_bounds.w() as f32) / 2.0, -(some_bounds.h() as f32) / 2.0);

        let p = pointer::world();

        let is_btn_over = p.intersects(some_bounds.x(), some_bounds.y(), some_bounds.w(), some_bounds.h());

        if is_btn_over {
            
            if p.released() {

                self.handle_hover(_ent, _state);

            }
            else {

                self.handle_press(_ent, _state);

            }

        }
        else if self.state != ButtonStates::None {
            
            self.handle_away(_ent, _state);

        }
        else {
            self.on_away(_ent, _state);
        }

    }

    pub fn render(&self, _transform : Transform, _state : &mut GameState) {

        let mut some_bounds = Bounds::with_size(self.transform.get_width() as f32 * self.transform.get_scale_x() * _transform.get_scale_x(), 
                                            self.transform.get_height() as f32 * self.transform.get_scale_y() * _transform.get_scale_y());

        let offset = self.transform.get_xy_offset(false);

        some_bounds = some_bounds.position_xy((offset.0 + _transform.get_x_offset(), offset.1 + _transform.get_y_offset()));
        some_bounds = some_bounds.translate(-(some_bounds.w() as f32) / 2.0, -(some_bounds.h() as f32) / 2.0);

        // Renders a shadow if necessary

        if self.has_shadow() {

            rect!(
                color = 0x000000ff,
                x = some_bounds.x() + 2,
                y = some_bounds.y() + 2,
                w = some_bounds.w() + 1,
                h = some_bounds.h() + 1,
                border_size = self.border.get_size() * self.transform.get_scale() as u32,
                border_color = 0x000000ff,
                border_radius = self.border.get_radius()
            );

        }

        // Renders the main button

        rect!(
            color = self.color,
            x = some_bounds.x(),
            y = some_bounds.y(),
            w = some_bounds.w(),
            h = some_bounds.h(),
            border_size = self.border.get_size() * self.transform.get_scale() as u32,
            border_color = self.border.get_color(),
            border_radius = self.border.get_radius()
        );

    }

    pub fn has_shadow(&self) -> bool {

        if self.state == ButtonStates::None {
            return false;
        }

        match &self.button_type {
            _default => {return false;}
        }

    }

}

impl ButtonComponent {

    pub fn handle_hover(&mut self, _ent : &mut Entity, _state : &mut GameState) {

        self.hov_count += 1;

        if self.state == ButtonStates::None {

            self.state = ButtonStates::Hover;
            self.on_enter(_ent, _state);

            self.hov_count = 1;

        }
        else if self.state == ButtonStates::Press{
            
            self.state = ButtonStates::Hover;
            
            self.on_release(_ent, _state);
            self.push_count = 0;

        }
        else {
            self.on_hover(_ent, _state);
        }
                
    }

    pub fn handle_press(&mut self, _ent : &mut Entity, _state : &mut GameState) {

        let p = pointer::screen();

        self.push_count += 1;

        if p.just_pressed() {
            self.on_click(_ent, _state);

            // Adding the destroy the notif hot fix

            self.state = ButtonStates::Press;

            self.push_count = 1;

        }
        else if self.state == ButtonStates::Press{
            self.on_hold(_ent, _state);
        }

    }

    pub fn handle_away(&mut self, _ent : &mut Entity, _state : &mut GameState) {

        // Add state specific stuff for the end later

        if self.state == ButtonStates::Press {
            self.on_release(_ent, _state);
        }

        self.on_exit(_ent, _state);

        self.state = ButtonStates::None;
        self.sub_state = SubStates::None;

        self.hov_count = 0;
        self.push_count = 0;

    }

}

impl ButtonComponent {

    // Hover based functions

    pub fn on_enter(&mut self, _ent : &mut Entity, _state : &mut GameState) {

        match &self.button_type {
            _default => {

            }
        }

    }

    pub fn on_hover(&mut self, _ent : &mut Entity, _state : &mut GameState) {
        
        match &self.button_type {
            _default => {

            }
        }

    }

    pub fn on_exit(&mut self, _ent : &mut Entity, _state : &mut GameState) {
        
        match &self.button_type {
            _default => {

            }
        }

    }

    // Click sensitive functions

    pub fn on_click(&mut self, _ent : &mut Entity, _state : &mut GameState) {

        match &self.button_type {

            // Testing

            ButtonTypes::Misc => {
                to_misc::on_click(self, _ent, _state);
            },
            ButtonTypes::Title => {
                to_title::on_click(self, _ent, _state);
            },

            _default => {

            }
        }

    }

    pub fn on_hold(&mut self, _ent : &mut Entity, _state : &mut GameState) {
        
        match &self.button_type {
            _default => {

            }
        }

    }

    pub fn on_release(&mut self, _ent : &mut Entity, _state : &mut GameState) {

        match &self.button_type {
            _default => {

            }
        }

    }

    // The not over case

    pub fn on_away(&mut self, _ent : &mut Entity, _state : &mut GameState){

        match &self.button_type {
            _default => {

            }
        }

    }
    
}

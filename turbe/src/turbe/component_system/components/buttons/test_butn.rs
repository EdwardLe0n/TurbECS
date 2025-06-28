use turbo::*;

// Core directories

use crate::turbe;

use crate::GameState;

// Necessary imports

use turbe::entity::Entity;
use turbe::component_system;

use component_system::components::comp_butn::ButtonComponent;

// Hover based functions

pub fn on_enter (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!
    _button.color = 0x999999ff;

}

pub fn on_hover (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

    if !audio::is_playing("cat_waimf") {
        audio::play("cat_waimf");
    }

}

pub fn on_exit (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    audio::stop("cat_waimf");
    _button.color = 0xffffffff;

}

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

    _button.color = 0x777777ff; 
    audio::play("glorp");

}

pub fn on_hold (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

    _state.test_var += 1;

}

pub fn on_release (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

    _button.color = 0xffffffff;

}

// The not over case

pub fn on_away (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!
    _state.test_var -= 1;

}
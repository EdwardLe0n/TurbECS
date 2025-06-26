use turbo::prelude::*;

// Core directories

use crate::turbe;

use crate::GameState;

// Necessary imports

use turbe::entity::Entity;
use turbe::component_system;

use component_system::components::comp_butn::ButtonComponent;
use component_system::component::Component;

// Hover based functions

pub fn on_enter (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

}

pub fn on_hover (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

}

pub fn on_exit (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

}

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

}

pub fn on_hold (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

}

pub fn on_release (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

}

// The not over case

pub fn on_away (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

}
use turbo::*;

// Core directories

use crate::turbe;

use crate::GameState;

// Necessary imports

use turbe::entity::Entity;
use turbe::component_system;
use turbe::scene_data::Scenes;

use component_system::components::comp_butn::ButtonComponent;
use component_system::component::Component;

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // Toss some code in here!

    _state.scene_data.load_scene(Scenes::Battle);

}
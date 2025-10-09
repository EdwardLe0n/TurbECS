// Core directories

use turbo::*;

use crate::turbecs;
use crate::GameState;

// Necessary imports

use turbecs::entity::Entity;
use turbecs::component_system;
use turbecs::scene_data::Scenes;

use component_system::components::comp_butn::ButtonComponent;
use component_system::component_types::ComponentTypes;
use component_system::component::ComponentData;

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    // First checks if we can go right any further

    // LATER

    // does screen manager stuff

    let ent_locat = _state.find_w_component(ComponentTypes::ScreenManager);

    if !ent_locat.0 {
        return;
    }

    let screen_ent = _state.entity_manager.entities[ent_locat.1].clone();

    let locat = screen_ent.find_component(ComponentTypes::ScreenManager, _state);

    if !locat.0 {
        return;
    }

    if let ComponentData::ScreenManager(screen_manager_data) = &mut _state.component_manager.components[screen_ent.comp_locats[locat.1]].component_data.clone() {

        if screen_manager_data.start_swipe(_state, true){

            _state.component_manager.components[screen_ent.comp_locats[locat.1]].component_data = ComponentData::ScreenManager(screen_manager_data.clone());

        }

    }

}
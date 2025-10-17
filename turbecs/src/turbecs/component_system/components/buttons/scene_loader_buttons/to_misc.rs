// Core directories

use crate::turbecs;

use crate::GameState;

// Necessary imports

use turbecs::entity::Entity;
use turbecs::component_system;
use turbecs::managers::scene_manager::Scenes;

use component_system::components::comp_butn::ButtonComponent;

// Click sensitive functions

pub fn on_click (_button : &mut ButtonComponent, _ent : &mut Entity, _state : &mut GameState) {

    _state.scene_manager.load_scene(Scenes::Misc);

}
// Necessary imports

use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;

use crate::kitchen::objectSystem::GameObject;
use crate::kitchen::{objectManager, prefabs};
use crate::GameState;

#[derive(Debug, Clone, Copy, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Screen {
    Title,
    MainMenu,
    Credits,
}

pub fn check_scene() {

    let mut state = GameState::load();

    // Checks if the screen data has been loaded, and if not, it will do so now!
    if !state.scene_data.loaded {

        log!("New screen needs the new objects!");

        state.current_game_objects.clear();

        state.save();
        state = GameState::load();

        objectManager::new_game_object_list(load_scene_data(state.scene_data.screen));

        state = GameState::load();

        state.scene_data.loaded = true;

        state.save();

    }

}

pub fn load_scene_data(current_screen : Screen) -> Vec<GameObject>{

    let mut object_list : Vec<GameObject> = Vec::new();

    // log!("elements are being gathered!");

    // Switch case that gets elements based on all of the possible screens
    match current_screen {
        Screen::Title => {object_list = load_title_scene()}
        Screen::MainMenu => {}
        Screen::Credits => {}
    }

    // log!("elements are being returned");

    return object_list;

}

pub fn load_title_scene() -> Vec<GameObject>{

    let mut object_list : Vec<GameObject> = Vec::new();

    object_list.push(prefabs::title());

    object_list.push(prefabs::test_rect());
    object_list.push(prefabs::test_rect_2());

    // object_list.push(prefabs::square());
    
    // object_list.push(prefabs::moving_cam());

    // object_list.push(prefabs::player());

    return object_list;

}
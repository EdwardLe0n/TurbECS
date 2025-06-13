// Necessary imports

use std::usize;

use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;

use crate::GameState;

//use super::componentTypes;
// use componentTypes::ComponentTypes;

// Base model????
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Target {

    pub(super) target_stored : bool,
    pub(super) target_name : String,
    pub(super) targetting_game_object : bool,
    // pub(super) target_component : ComponentTypes,
    pub(super) target_locat : Vec<usize>
    
}

impl Target {

    pub fn new() -> Target{
 
        return Target {
            target_stored : false,
            target_name : "temp".to_string(),
            targetting_game_object : false,
            target_component : ComponentTypes::None,
            target_locat : Vec::new()
        };

    }

    // // Makes a parent target base then returns the target
    // pub fn make_parent(some_place : usize) -> Target
    // {

    //     let mut temp : Target = Target::new();

    //     // hehe
    //     // Sets the stored state
    //     temp.target_stored = true;

    //     // Loads the game state to nab the name of the object

    //     let state = GameState::load();
    //     temp.target_name = state.current_game_objects[some_place].get_name().clone();
    //     state.save();

    //     // Mentions the fact that since since it's targetting a parent,
    //     temp.targetting_game_object = true;

    //     // Tosses in the target location of the game object
    //     temp.target_locat.push(some_place);

    //     return temp;

    // }

    // Makes a reference of the current gameobject/component
    pub fn make_ref() {

    }
    
}

/* Setters and Getters */

impl Target {
    
    pub fn set_target_name(&mut self, temp_string : String) {
        self.target_name = temp_string;
    }

    pub fn get_target_name(self) -> String {
        return self.target_name;
    }

}
use turbo::prelude::*;

use crate::GameState;

use super::objectSystem;
use objectSystem::GameObject;

pub fn new_game_object_list(some_game_object_vec : Vec<GameObject>) {

    for i in 0..some_game_object_vec.len() {

        new_game_object(some_game_object_vec[i].clone());

    }

}

pub fn new_game_object (some_game_object : GameObject) {

    let mut state = GameState::load();
    let mut temp_usize: usize = 0;

    // first checks if there are any empty spaces available
    // if so then it will chuck the game object there

    if state.gap_data.empty_spaces.len() != 0 {

        temp_usize = state.gap_data.empty_spaces[0];
        state.gap_data.empty_spaces.pop_front();

        state.current_game_objects[temp_usize] = some_game_object;

    }

    // if not, it will just add it to the end of the vec

    else {

        temp_usize = state.current_game_objects.len();

        state.current_game_objects.push(some_game_object);

    }

    // will also keep track of the recently filled location!
    state.gap_data.recently_filled.push_back(temp_usize.clone());

    state.save();

}

// pub fn find_camera() -> (i32, i32) {

//     let mut state : GameState = GameState::load();

//     for i in 0.. state.current_game_objects.len() {

//         for j in 0.. state.current_game_objects[i].components.len()
//         {

//             if (state.current_game_objects[i].components[j].comp_type == ComponentTypes::Camera)
//             {
//                 return (state.current_game_objects[i].get_pos_x(), state.current_game_objects[i].get_pos_y());
//             }

//         }

//     }

//     log!("made a new camera");

//     let temp : GameObject = GameObject::new_camera(); 

//     state.current_game_objects.push(temp);

//     state.save();

//     return find_camera();

//}

pub fn manage_new_objects(){

    let mut state: GameState = GameState::load();

    if state.gap_data.recently_filled.len() > 0 {

        log!("Handling new object func for {:?} objects", state.gap_data.recently_filled.len());

        while state.gap_data.recently_filled.len() > 0 {

            state.current_game_objects[state.gap_data.recently_filled[0]]
                .on_init(state.gap_data.recently_filled[0], 
                            state.gap_data.recently_filled[0]);

            state = GameState::load();

            state.lifetime_cycle.new_awake.push_back(state.gap_data.recently_filled[0]);
            state.gap_data.recently_filled.pop_front();

            state.save();
            state = GameState::load();

        }

    }

    state.save();
    
}

pub fn manage_awake(){

    let mut state: GameState = GameState::load();

    if state.lifetime_cycle.new_awake.len() > 0{

        log!("Managing awake function! {:?}", state.lifetime_cycle.new_awake.len());

        while state.lifetime_cycle.new_awake.len() > 0 {

            state.current_game_objects[state.lifetime_cycle.new_awake[0]].on_awake();

            state = GameState::load();

            state.lifetime_cycle.new_start.push_back(state.lifetime_cycle.new_awake[0]);
            state.lifetime_cycle.new_awake.pop_front();

            state.save();
            state = GameState::load();

        }
    }

    state.save();

    // log!("Completed manage new awake");
    
}

pub fn manage_start() {

    let mut state: GameState = GameState::load();

    while state.lifetime_cycle.new_start.len() > 0 {

        state.current_game_objects[state.lifetime_cycle.new_start[0]].on_awake();

        state = GameState::load();

        state.lifetime_cycle.new_start.pop_front();

        state.save();
        state = GameState::load();

    }

    state.save();

}

pub fn manage_update(){

    let mut state: GameState = GameState::load();

    for i in 0.. state.current_parent_objects.len() {
        for j in 0.. state.current_parent_objects[i].len() {

            // log!("{:?} elements in this row", j);

            state.current_game_objects[state.current_parent_objects[i][j]].on_update();
            // state = GameState::load();

        }
    }

    state.save();

}

pub fn manage_destroy() {

}

pub fn manage_render(){

    // let locat : (i32, i32) = find_camera();

    let locat : (i32,i32) = (0,0);

    // log!("coords {:?}, {:?}", locat.0, locat.1);

    let mut state = GameState::load();

    // log!("{:?} is the number of parent game objects!", debug_get_parent_game_object_count());

    state = GameState::load();

    clear(0x3f5270ff);

    camera::set_xy(locat.0, locat.1);

    // Sanity 
    // text!("(0, 0)");

    for i in 0.. state.current_parent_objects.len() {

        for j in 0.. state.current_parent_objects[i].len() {
            
            state.current_game_objects[state.current_parent_objects[i][j]].on_render(locat.0, locat.1);
            state = GameState::load();

        }

    }

    state.save();
    

}



pub fn debug_get_parent_game_object_count() -> i32 {

    let mut count : i32 = 0;

    let state = GameState::load();

    for i in 0..state.current_parent_objects.len() {
        for j in 0..state.current_parent_objects[i].len() {
            count += 1;
        }
    }

    return count;

}
// Initial imports

mod turbe;
// mod assets;
// use kitchen::objectManager;

// use crate::kitchen::objectSystem::GameObject;
use crate::turbe::sceneManagementSystem;
// use crate::kitchen::sceneManagementSystem::Screen;

use std::collections::VecDeque;

turbo::init! {
    
    // Define the GameState struct.
    struct GameState {
        
        // This screen bit as a base, acts a potential scene system
        
        scene_data: struct SceneData {

            loaded : bool,
            //screen: Screen

        } ,
        lifetime_cycle : struct LifetimeCycleObjects {
            new_awake : VecDeque<usize>,
            new_start : VecDeque<usize>,
            new_destroy : VecDeque<usize>
        },
        gap_data : struct GapData { 
            empty_spaces : VecDeque<usize>,
            recently_filled : VecDeque<usize>,
        },
        //current_game_objects : Vec<GameObject>,
        current_parent_objects : Vec<Vec<usize>> 
        
    } = {
        // Set the structs initial value.
        Self {
            scene_data : SceneData { 
                loaded: (false), 
                // screen: Screen::Title 
            },
            lifetime_cycle : LifetimeCycleObjects {
                new_awake : VecDeque::new(),
                new_start : VecDeque::new(),
                new_destroy : VecDeque::new()
            },
            gap_data : GapData {
                empty_spaces : VecDeque::new(),
                recently_filled : VecDeque::new()
            },
            //current_game_objects : Vec::new(),
            current_parent_objects : Vec::new()
        }
    }
    
}

// This is where your main game loop code goes
// The stuff in this block will run ~60x per sec
turbo::go!({

    text!("Hello, world!!!");

});
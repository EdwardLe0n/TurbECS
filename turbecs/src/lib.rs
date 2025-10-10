
// Core Turbo functionality
use turbo::*;

// Links the driver file to the main lib file
mod ecs_gamestate;

// Initial TurbECS import
mod turbecs;

// Manager imports
use turbecs::{managers};

use managers::entity_manager::EntityManager;
use managers::component_manager::ComponentManager;
use managers::scene_manager::SceneManager;

// Community work/manager imports
use managers::particlemanager::ParticleManager;

// Game specific elements that need to be in the GameState

mod assets;
use assets::game_state::{run_data::RunData};


#[turbo::game]
struct GameState {
    
    // Core managers for turbecs

    pub scene_manager : SceneManager,
    pub entity_manager : EntityManager,
    pub component_manager : ComponentManager,
    pub render_manager : Vec<Vec<usize>>,

    // Community integrated work/managers

    pub particle_manager : ParticleManager,

    // Project specific elements

    pub run_data : RunData,
    pub can_interact : bool

}

impl GameState {
    fn new() -> Self {

        camera::set_xy(0, 0);

        Self {
            
            // Core managers for turbecs

            scene_manager : SceneManager::new(),
            entity_manager : EntityManager::new(),
            component_manager : ComponentManager::new(),
            render_manager : Vec::with_capacity(10),
            
            // Community integrated work/managers

            particle_manager : ParticleManager::new(),

            // Project specific elements

            run_data : RunData::new(),
            can_interact : true
        }
    
    }

    fn update(&mut self) {

        // Checks the scene state before continuing

        self.check_scene_state();

        // From here TurbECS will run it's lifetime functions!

        self.run_lifetime();

    }
}
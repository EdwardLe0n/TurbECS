use turbo::*;

// Initial imports

use std::{collections::VecDeque};

mod turbecs;
use turbecs::{entity::Entity, scene_data, managers};
use turbecs::{particles::ParticleManager};

use managers::{entity_manager::EntityManager};

use scene_data::{SceneData, Scenes};

mod assets;

use crate::turbecs::component_system;
use component_system::{component::Component, component_types::ComponentTypes};
use crate::turbecs::helpers::active_states::ActiveStates;

use assets::game_state::{run_data::RunData};

#[turbo::game]
struct GameState {
    
    pub scene_data : SceneData,
    pub entity_manager : EntityManager,
    pub render_manager : Vec<Vec<usize>>,

    // Additional libraries/manager

    pub particle_manager : ParticleManager,

    // specfic game stuff

    pub run_data : RunData,
    pub can_interact : bool

}

impl GameState {
    fn new() -> Self {

        camera::set_xy(0, 0);

        Self {scene_data : SceneData { active_scene: (Scenes::Misc), is_loaded: (false) },
            entity_manager : EntityManager::new(), 
            render_manager : Vec::with_capacity(10),
            particle_manager : ParticleManager::new(),
            run_data : RunData::new(), can_interact : true}
    
    }

    fn update(&mut self) {
        // Update the game & draw stuff

        self.check_scene_state();

        self.run_lifetime();

    }
}

/*

    Scene Systems!!!

*/

impl GameState {
    
    fn check_scene_state(&mut self) {

        if self.scene_data.is_loaded {

            return;

        }

        for i in 0..self.entity_manager.entities.len() {
            if (self.entity_manager.entities[i].state != ActiveStates::Destroyed) {
                self.entity_manager.lifetime_data.new_destroy.push_back(i);
            }
        }

        // Sanity check

        log!("number of entities: {:?}", self.entity_manager.entities.len());
        log!("number of entities to be destroyed : {:?}", self.entity_manager.lifetime_data.new_destroy.len());

        self.on_destroy();

        let mut new_ent = scene_data::make_scene(self.scene_data.active_scene);

        
        // Sanity check
        log!("number of new entities : {:?}", new_ent.len());

        self.new_entities(&mut new_ent);


        self.scene_data.is_loaded = true;

        let len = borsh::to_vec(self).unwrap().len();
        log!("LEN = {len}");

    } 

    pub fn new_entities(&mut self, _entities : &mut VecDeque<(Entity, Vec<Component>)>) {

        while !_entities.is_empty()
        {

            let some_ent = &mut _entities.front().unwrap().0.clone();
            _entities.pop_front();

            for c in &mut some_ent.components {
                c.init_has_x();
            }

            some_ent.init_has_x();

            self.new_entity(some_ent);

        }
        
    }

    pub fn new_entity(&mut self, _entity : &mut Entity) {

        self.entity_manager.new_entities.push(_entity.clone());

    }

    fn get_num_of_free_locat(&mut self) -> usize {
        return self.entity_manager.gap_data.empty_spaces.len();
    }

    fn get_next_free(&mut self) -> usize {

        let next = *self.entity_manager.gap_data.empty_spaces.front().unwrap();

        self.entity_manager.gap_data.empty_spaces.pop_front();

        return next;

    }

}

/*

    Lifetime System!!!

*/

impl GameState {

    fn run_lifetime(&mut self) {

        self.load_entities();

        self.on_awake();

        self.on_start();

        self.on_update();

        self.on_destroy();

        self.on_render();

    }

    pub fn load_entities(&mut self) {

        if self.entity_manager.new_entities.len() == 0{
            return;
        }

        for ent in self.entity_manager.new_entities.clone() {
            self.load_entity(&mut ent.clone());
        } 

        self.entity_manager.new_entities.clear();

    }

    pub fn load_entity(&mut self, _entity : &mut Entity) {
        let mut next : usize = 0;

        if self.get_num_of_free_locat() > 0 {

            next = self.get_next_free();
            _entity.locat = next.clone();

            self.entity_manager.entities[next] = _entity.clone();

            // do linkage here!

        }
        else {

            next = self.entity_manager.entities.len();
            _entity.locat = next.clone();

            self.entity_manager.entities.push(_entity.clone());

            // do linkage here!

        }

        self.entity_manager.lifetime_data.new_awake.push_back(next);
    }

}

/*
* Helper functions 
*/

// Misc

impl GameState {
    
    pub fn find_w_component(&mut self, some_type : ComponentTypes) -> (bool, usize) {

        for i in 0..self.entity_manager.entities.len() {

            if !self.entity_manager.entities[i].is_destroyed() {

                if self.entity_manager.entities[i].has_component(some_type.clone()) {
                    return (true, i);
                }

            }

        }

        return (false, 0);

    }

}
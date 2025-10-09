use turbo::*;

// Initial imports

use std::{collections::VecDeque};

mod turbecs;
use turbecs::{entity::Entity, managers};
use turbecs::{particles::ParticleManager};

use managers::{entity_manager::EntityManager, component_manager::ComponentManager, scene_manager};

use scene_manager::{SceneManager, Scenes};

mod assets;

use crate::turbecs::component_system;
use component_system::{component::Component, component_types::ComponentTypes};
use crate::turbecs::helpers::active_states::ActiveStates;

use assets::game_state::{run_data::RunData};

#[turbo::game]
struct GameState {
    
    pub scene_manager : SceneManager,
    pub entity_manager : EntityManager,
    pub component_manager : ComponentManager,
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

        Self {scene_manager : SceneManager::new(),
            entity_manager : EntityManager::new(),
            component_manager : ComponentManager::new(),
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

        if self.scene_manager.is_loaded {

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

        let mut new_ent = scene_manager::make_scene(self.scene_manager.active_scene);

        
        // Sanity check
        log!("number of new entities : {:?}", new_ent.len());

        self.new_entities(&mut new_ent);

        self.scene_manager.is_loaded = true;

        let len = borsh::to_vec(self).unwrap().len();
        log!("LEN = {len}");

    } 

    pub fn new_entities(&mut self, _entities : &mut VecDeque<(Entity, VecDeque<Component>)>) {

        while !_entities.is_empty()
        {

            let mut some_ent = _entities.front().unwrap().clone();
            _entities.pop_front();

            while !some_ent.1.is_empty() {

                some_ent.0.comp_locats.push(self.component_manager.next_comp_locat().1);

                self.component_manager.new_component(some_ent.1.front().unwrap().clone());
                some_ent.1.pop_front();

            }

            for c in &some_ent.0.comp_locats {
                self.component_manager.components[*c].init_has_x();
            }

            some_ent.0.init_has_x(self);

            self.new_entity(&mut some_ent.0);

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

                let ent =  self.entity_manager.entities[i].clone();

                if ent.has_component(some_type.clone(), self) {
                    return (true, i);
                }

            }

        }

        return (false, 0);

    }

}
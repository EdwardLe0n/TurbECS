use turbo::*;

// Initial imports

use std::{collections::VecDeque};

mod turbecs;
use turbecs::{entity::Entity, scene_data, gap_data::GapData, lifetime_data::LifetimeData, managers};
use turbecs::{particles::ParticleManager};

use managers::{entity_manager::EntityManager};

use scene_data::{SceneData, Scenes};

mod assets;

use crate::turbecs::component_system::component_types::ComponentTypes;
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

    pub fn new_entities(&mut self, _entities : &mut VecDeque<Entity>) {

        while !_entities.is_empty()
        {

            let mut some_ent = _entities.front().unwrap().clone();
            _entities.pop_front();

            self.new_entity(&mut some_ent);

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

        // Sanity
        // log!("Attempting the load");

        self.load_entities(); 

        // Sanity
        // log!("At awake");

        self.on_awake();

        // Sanity
        // log!("At start");

        self.on_start();

        // Sanity
        // log!("At update");

        self.on_update();

        // Sanity
        // log!("At destroy");

        self.on_destroy();

        // Sanity
        // log!("At render");

        self.on_render();

    }

    pub fn load_entities(&mut self) {

        if self.entity_manager.new_entities.len() == 0{
            return;
        }

        for ent in self.entity_manager.new_entities.clone() {

            // Sanity
            // log!("Loading new");

            self.load_entity(&mut ent.clone());
        } 

        self.entity_manager.new_entities.clear();

    }

    pub fn load_entity(&mut self, _entity : &mut Entity) {
        let mut next : usize = 0;

        if self.get_num_of_free_locat() > 0 {

            next = self.get_next_free();
            _entity.locat = next.clone();

            // Sanity
            // log!("replacing something");

            self.entity_manager.entities[next] = _entity.clone();

        }
        else {

            next = self.entity_manager.entities.len();
            _entity.locat = next.clone();

            // Sanity
            // log!("Adding something");

            self.entity_manager.entities.push(_entity.clone());

        }

        // Sanity
        // log!("Gonna log entity {} at {}", self.entities[next].name, next);

        self.entity_manager.lifetime_data.new_awake.push_back(next);
    }

    fn on_awake(&mut self) {

        let len = self.entity_manager.lifetime_data.new_awake.len();

        if len == 0 {
            return;
        }

        // Sanity
        // log!("Going to awake {} entities with a total of {} entities", len, self.entities.len());

        for _i in 0..len {

            // Sanity
            // log!("Awakening {} with name {}", *self.lifetime_data.new_awake.front().unwrap(), "test");

            let test_val = *self.entity_manager.lifetime_data.new_awake.front().unwrap();

            // Sanity
            // log!("{:?}", test_val);

            let mut some_ent = self.entity_manager.entities[test_val].clone();

            some_ent.on_awake(self);

            self.entity_manager.entities[test_val] = some_ent;

            self.entity_manager.lifetime_data.new_start.push_back(*self.entity_manager.lifetime_data.new_awake.front().unwrap());
            self.entity_manager.lifetime_data.new_awake.pop_front();

        }

    }

    fn on_start(&mut self) {

        let len = self.entity_manager.lifetime_data.new_start.len();

        if len == 0 {
            return;
        }

        for _i in 0..len {

            let locat = *self.entity_manager.lifetime_data.new_start.front().unwrap();

            let mut some_ent = self.entity_manager.entities[locat].clone();

            some_ent.on_start(self);

            self.entity_manager.entities[locat] = some_ent;

            self.entity_manager.lifetime_data.new_start.pop_front();

        }

    }

    // Loops through all of the entities in the entity vector in order of newest to oldest
    fn on_update(&mut self) {

        let len = self.entity_manager.entities.len();

        for i in 0..len {

            let mut some_ent = self.entity_manager.entities[i].clone();

            some_ent.on_update(self);

            self.entity_manager.entities[i] = some_ent;

        }

        // particles!!!!

        self.particle_manager.update();

    }

    fn on_destroy(&mut self) {

        let len = self.entity_manager.lifetime_data.new_destroy.len();

        if len == 0 {
            return;
        }

        for _i in 0..len {

            let locat = *self.entity_manager.lifetime_data.new_destroy.front().unwrap();

            let mut some_ent = self.entity_manager.entities[locat].clone();

            if !some_ent.is_destroyed() {

                some_ent.on_destroy(self);
                self.entity_manager.entities[locat] = some_ent;

                self.entity_manager.gap_data.empty_spaces.push_back(locat);
                self.entity_manager.lifetime_data.new_destroy.pop_front();
                    
            }

        }

    }

    fn on_render(&mut self) {

        clear(0xeeeeeeff);

        let render_list = self.render_manager.clone();
        let entities = self.entity_manager.entities.clone();

        for i in 0..render_list.len() {

            for j in 0..render_list[i].len(){

                entities[render_list[i][j]].on_render(self);

            }
        }

        self.particle_manager.draw();

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
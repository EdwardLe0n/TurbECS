use turbo::*;

// Initial imports

use std::{collections::VecDeque};

mod turbecs;
use turbecs::{entity::Entity, scene_data, component_system, gap_data::GapData, lifetime_data::LifetimeData};
use turbecs::{particles::ParticleManager};

use component_system::{component::Component};
use scene_data::{SceneData, Scenes};

mod assets;

use crate::turbecs::component_system::component_types::ComponentTypes;
use crate::turbecs::helpers::active_states::ActiveStates;

use assets::game_state::{run_data::RunData};

#[turbo::game]
struct GameState {
    
    pub scene_data : SceneData,
    pub gap_data : GapData,
    pub lifetime_data : LifetimeData,
    pub entities : Vec<Entity>,
    pub new_entities : Vec<Entity>,
    pub render_list : Vec<Vec<usize>>,

    // code from other peeps

    pub particle_manager : ParticleManager,

    // specfic game stuff

    pub run_data : RunData,
    pub can_interact : bool

}

impl GameState {
    fn new() -> Self {

        let b_tree = Vec::new();
        let render_l = Vec::new();

        camera::set_xy(0, 0);

        Self {scene_data : SceneData { active_scene: (Scenes::Title), is_loaded: (false) }, 
            gap_data : GapData::new(), lifetime_data : LifetimeData::new(),
            entities : b_tree, new_entities : Vec::new(), render_list : render_l,
            particle_manager : ParticleManager::new(),
            run_data : RunData::new(), can_interact : true}
    
    }

    fn update(&mut self) {
        // Update the game & draw stuff


        // Sanity

        // let len = borsh::to_vec(self).unwrap().len();
        // log!("LEN = {len}");

        self.check_scene_state();

        // log!("starting liftime");

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

        for i in 0..self.entities.len() {
            if (self.entities[i].state != ActiveStates::Destroyed) {
                self.lifetime_data.new_destroy.push_back(i);
            }
        }

        // Sanity check

        log!("number of entities: {:?}", self.entities.len());
        log!("number of entities to be destroyed : {:?}", self.lifetime_data.new_destroy.len());

        self.on_destroy();

        let mut new_ent = scene_data::make_scene(self.scene_data.active_scene);

        log!("number of new entities : {:?}", new_ent.len());

        // Sanity check
        self.new_entities(&mut new_ent);

        // Sanity
        // log!("done done");

        self.scene_data.is_loaded = true;

        // Sanity
        // log!("done done done");

        let len = borsh::to_vec(self).unwrap().len();
        log!("LEN = {len}");

    } 

    pub fn new_entities(&mut self, _entities : &mut VecDeque<Entity>) {

        // Sanity
        // log!("Trying the loop");

        while !_entities.is_empty()
        {

            // Sanity
            // log!("Trying something");

            let mut some_ent = _entities.front().unwrap().clone();
            _entities.pop_front();

            self.new_entity(&mut some_ent);

        }

        // Sanity  
        // log!("Done");  
        
    }

    pub fn new_entity(&mut self, _entity : &mut Entity) {

        self.new_entities.push(_entity.clone());

    }

    fn get_num_of_free_locat(&mut self) -> usize {
        return self.gap_data.empty_spaces.len();
    }

    fn get_next_free(&mut self) -> usize {

        let next = *self.gap_data.empty_spaces.front().unwrap();

        self.gap_data.empty_spaces.pop_front();

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

        if self.new_entities.len() == 0{
            return;
        }

        for ent in self.new_entities.clone() {

            // Sanity
            // log!("Loading new");

            self.load_entity(&mut ent.clone());
        } 

        self.new_entities.clear();

    }

    pub fn load_entity(&mut self, _entity : &mut Entity) {
        let mut next : usize = 0;

        if self.get_num_of_free_locat() > 0 {

            next = self.get_next_free();
            _entity.locat = next.clone();

            // Sanity
            // log!("replacing something");

            self.entities[next] = _entity.clone();

        }
        else {

            next = self.entities.len();
            _entity.locat = next.clone();

            // Sanity
            // log!("Adding something");

            self.entities.push(_entity.clone());

        }

        // Sanity
        // log!("Gonna log entity {} at {}", self.entities[next].name, next);

        self.lifetime_data.new_awake.push_back(next);
    }

    fn on_awake(&mut self) {

        let len = self.lifetime_data.new_awake.len();

        if len == 0 {
            return;
        }

        // Sanity
        // log!("Going to awake {} entities with a total of {} entities", len, self.entities.len());

        for _i in 0..len {

            // Sanity
            // log!("Awakening {} with name {}", *self.lifetime_data.new_awake.front().unwrap(), "test");

            let test_val = *self.lifetime_data.new_awake.front().unwrap();

            // Sanity
            // log!("{:?}", test_val);

            let mut some_ent = self.entities[test_val].clone();

            some_ent.on_awake(self);

            self.entities[test_val] = some_ent;

            self.lifetime_data.new_start.push_back(*self.lifetime_data.new_awake.front().unwrap());
            self.lifetime_data.new_awake.pop_front();

        }

    }

    fn on_start(&mut self) {

        let len = self.lifetime_data.new_start.len();

        if len == 0 {
            return;
        }

        for _i in 0..len {

            let locat = *self.lifetime_data.new_start.front().unwrap();

            let mut some_ent = self.entities[locat].clone();

            some_ent.on_start(self);

            self.entities[locat] = some_ent;

            self.lifetime_data.new_start.pop_front();

        }

    }

    fn on_update(&mut self) {

        let len = self.entities.len();

        for i in 0..len {

            let mut some_ent = self.entities[i].clone();

            some_ent.on_update(self);

            self.entities[i] = some_ent;

        }

        // particles!!!!

        self.particle_manager.update();

    }

    fn on_destroy(&mut self) {

        let len = self.lifetime_data.new_destroy.len();

        if len == 0 {
            return;
        }

        for _i in 0..len {

            let locat = *self.lifetime_data.new_destroy.front().unwrap();

            let mut some_ent = self.entities[locat].clone();

            if !some_ent.is_destroyed() {

                some_ent.on_destroy(self);
                self.entities[locat] = some_ent;

                self.gap_data.empty_spaces.push_back(locat);
                self.lifetime_data.new_destroy.pop_front();
                    
            }

        }

    }

    fn on_render(&mut self) {

        clear(0xeeeeeeff);

        let render_list = self.render_list.clone();
        let entities = self.entities.clone();

        for i in 0..render_list.len() {

            for j in 0..render_list[i].len(){

                entities[render_list[i][j]].on_render(self);

            }
        }

        self.particle_manager.draw();

    }

}

// Misc

impl GameState {
    
    pub fn find_w_component(&mut self, some_type : ComponentTypes) -> (bool, usize) {

        for i in 0..self.entities.len() {

            if !self.entities[i].is_destroyed() {

                if self.entities[i].has_component(some_type.clone()) {
                    return (true, i);
                }

            }

        }

        return (false, 0);

    }

}
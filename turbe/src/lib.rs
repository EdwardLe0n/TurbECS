// Initial imports

use std::{collections::VecDeque, vec};

mod turbe;
use turbe::{entity::Entity, scene_data, component_system, gap_data::GapData, lifetime_data::LifetimeData};
use component_system::{component::Component};
use scene_data::{SceneData, Scenes};

mod assets;

use turbo::*;

use crate::turbe::helpers::active_states::ActiveStates;

#[turbo::game]
#[derive(PartialEq)]
struct GameState {
    
    pub scene_data : SceneData,
    pub gap_data : GapData,
    pub lifetime_data : LifetimeData,
    pub entities : Vec<Entity>,
    pub render_list : Vec<Vec<usize>>,

    pub test_var : i32

}

impl GameState {
    fn new() -> Self {

        let b_tree = Vec::new();
        let render_l = Vec::new();

        Self {scene_data : SceneData { active_scene: (Scenes::Title), is_loaded: (false) }, 
            gap_data : GapData::new(), lifetime_data : LifetimeData::new(),
            entities : b_tree, render_list : render_l , test_var : 0}
    
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

        self.scene_data.is_loaded = true;

    }

    pub fn new_entities(&mut self, _entities : &mut VecDeque<Entity>) {

        while _entities.len() > 0 
        {

            let mut some_ent = _entities.front().unwrap().clone();
            _entities.pop_front();

            self.new_entity(&mut some_ent);

        }
        
    }

    pub fn new_entity(&mut self, _entity : &mut Entity) {

        let mut next : usize = 0;

        if self.get_num_of_free_locat() > 0 {

            next = self.get_next_free();
            _entity.locat = next.clone();

            self.entities[next] = _entity.clone();

        }
        else {

            next = self.entities.len();
            _entity.locat = next.clone();

            self.entities.push(_entity.clone());

        }

        self.lifetime_data.new_awake.push_back(next);

        if self.render_list.len() <= self.entities[next].layer
        {
            while self.render_list.len() <= self.entities[next].layer {
                self.render_list.push(Vec::new());
            }
        }

        self.render_list[self.entities[next].layer].push(next);

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

        self.on_awake();
        self.on_start();
        self.on_update();
        self.on_destroy();
        self.on_render();

    }

    fn on_awake(&mut self) {

        let len = self.lifetime_data.new_awake.len();

        if len == 0 {
            return;
        }

        let mut entities = self.entities.clone();

        for _i in 0..len {

            entities[*self.lifetime_data.new_awake.front().unwrap()].on_awake(self);

            self.lifetime_data.new_start.push_back(*self.lifetime_data.new_awake.front().unwrap());
            self.lifetime_data.new_awake.pop_front();

        }

        self.entities = entities;

    }

    fn on_start(&mut self) {

        let len = self.lifetime_data.new_start.len();

        if len == 0 {
            return;
        }

        let mut entities = self.entities.clone();

        for _i in 0..len {

            entities[*self.lifetime_data.new_start.front().unwrap()].on_start(self);

            self.lifetime_data.new_start.pop_front();

        }

        self.entities = entities;

    }

    fn on_update(&mut self) {

        let mut entities = self.entities.clone();

        for entity in entities.iter_mut() {

            entity.on_update(self);

        }

        self.entities = entities;

    }

    fn on_destroy(&mut self) {

        let len = self.lifetime_data.new_destroy.len();

        if len == 0 {
            return;
        }

        let mut entities = self.entities.clone();

        for _i in 0..len {

            entities[*self.lifetime_data.new_destroy.front().unwrap()].on_destroy(self);

            self.gap_data.empty_spaces.push_back(*self.lifetime_data.new_destroy.front().unwrap());
            self.lifetime_data.new_destroy.pop_front();

        }

        self.entities = entities;

    }

    fn on_render(&mut self) {

        clear(0xaaaaaaff);

        let render_list = self.render_list.clone();
        let entities = self.entities.clone();

        for i in 0..render_list.len() {
            for j in 0..render_list[i].len(){

                entities[render_list[i][j]].on_render(self);

            }
        }

    }

}
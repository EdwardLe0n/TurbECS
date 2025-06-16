// Initial imports

use std::collections::VecDeque;
use std::collections::BTreeMap;
use std::vec;

mod turbe;
use turbe::{entity::Entity, scene_data, component::{Component, ComponentLifecycle}};
use scene_data::SceneData;

mod assets;
use assets::prefabs;

use turbo::{canvas::rect::Rectangle, prelude::*};

use crate::turbe::scene_data::Scenes;

#[turbo::game]
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
struct GameState {
    
    scene_data : SceneData,
    entities : Vec<Entity<Component>>,
    render_list : Vec<Vec<usize>>,

}

impl GameState {
    fn new() -> Self {

        let b_tree = Vec::new();
        let render_l = Vec::new();

        Self {scene_data : SceneData { active_scene: (Scenes::Title), is_loaded: (false) },entities : b_tree, render_list : render_l}
    
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

        let mut new_ent = scene_data::make_scene(self.scene_data.active_scene);

        while (new_ent.len() > 0) 
        {

            let mut some_ent = new_ent.front().unwrap().clone();
            new_ent.pop_front();

            self.entities.push(some_ent);

            if (self.render_list.len() <= self.entities[self.entities.len() - 1].layer)
            {
                while self.render_list.len() <= self.entities[self.entities.len() - 1].layer {
                    self.render_list.push(Vec::new());
                }
            }

            self.render_list[self.entities[self.entities.len() - 1].layer].push(self.entities.len() - 1);

        }

        self.scene_data.is_loaded = true;

    }

}

/*

    Lifetime System!!!

*/

impl GameState {

    fn run_lifetime(&mut self) {

        self.on_update();
        self.on_render();

    }

    fn on_update(&mut self) {

        for i in 0..self.entities.len() {

            let mut entDraft = self.entities[i].clone();

            for j in 0..self.entities[i].components.len() {

                self.entities[i].components[j].on_update(&mut entDraft);

            }

            self.entities[i].transform = entDraft.transform;

        }

    }

    fn on_render(&mut self) {

        clear(0xffffffff);

        for i in 0..self.render_list.len() {
            for j in 0..self.render_list[i].len(){
                for k in 0..self.entities[self.render_list[i][j]].components.len() {

                    let transform = self.entities[self.render_list[i][j]].transform.clone();

                    self.entities[self.render_list[i][j]].components[k].render(transform);

                    self.entities[self.render_list[i][j]].transform = transform;

                }
            }
        }

    }

}

// Other

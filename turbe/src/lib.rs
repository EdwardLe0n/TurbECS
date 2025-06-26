// Initial imports

use std::collections::VecDeque;
use std::vec;

mod turbe;
use turbe::{entity::Entity, scene_data, component_system};
use component_system::{component::Component, component_lifecycle::ComponentLifecycle};
use scene_data::{SceneData, Scenes};

mod assets;

use turbo::prelude::*;

#[turbo::game]
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
struct GameState {
    
    pub scene_data : SceneData,
    pub entities : Vec<Entity<Component>>,
    pub render_list : Vec<Vec<usize>>,

    pub test_var : i32

}

impl GameState {
    fn new() -> Self {

        let b_tree = Vec::new();
        let render_l = Vec::new();

        Self {scene_data : SceneData { active_scene: (Scenes::Title), is_loaded: (false) },entities : b_tree, render_list : render_l , test_var : 0}
    
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

        while new_ent.len() > 0 
        {

            let some_ent = new_ent.front().unwrap().clone();
            new_ent.pop_front();

            self.entities.push(some_ent);

            if self.render_list.len() <= self.entities[self.entities.len() - 1].layer
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

        let mut entities = self.entities.clone();

        for entity in entities.iter_mut() {

            let mut ent_draft = entity.clone();

            for j in 0..entity.components.len() {

                entity.components[j].on_update(&mut ent_draft, self);
                ent_draft.components[j] = entity.components[j].clone();

            }

            *entity = ent_draft;

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
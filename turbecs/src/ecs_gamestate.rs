use std::collections::VecDeque;

use turbo::*;

use crate::{turbecs, GameState};

use turbecs::managers::scene_manager;

use turbecs::entity::Entity;

use turbecs::component_system::{component::Component, component_types::ComponentTypes};
use crate::turbecs::helpers::active_states::ActiveStates;

/*

    Scene Systems!!!

*/

impl GameState {
    
    pub fn check_scene_state(&mut self) {

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

}

/*

    Lifetime System!!!

*/

impl GameState {

    pub fn run_lifetime(&mut self) {

        self.load_entities();

        self.on_awake();

        self.on_start();

        self.on_update();

        self.on_destroy();

        self.on_render();

    }

}

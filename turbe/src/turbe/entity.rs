use turbo::prelude::*;

use crate::{turbe, GameState};

use turbe::component_system::{component::Component};

use turbe::helpers;
use helpers::{transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Entity {
    pub name: String,
    pub components: Vec<Component>,
    pub transform: Transform,
    pub layer: usize,
    pub locat: usize,
}

impl Entity {

    pub fn new (name : String, vec : Vec<Component>) -> Self {

        Self { 
            name: name, components: vec, transform: Transform::new(), layer: 0, locat: 0 
        }

    }

    pub fn new_base (_name : String) -> Entity {

        return Entity::new(_name, vec![]);

    }

    pub fn add_component (&mut self, component : Component) {

        self.components.push(component);

    }

}

// Lifetime Systems

impl Entity {

    pub fn on_init(&mut self, _state : &mut GameState) {

        

    }

    pub fn on_awake(&mut self, _state : &mut GameState) {

        let mut ent_draft = self.clone();

        for i in 0..self.components.len() {

            self.components[i].on_awake(&mut ent_draft, _state);
            ent_draft.components[i] = self.components[i].clone();

        }

        *self = ent_draft;
    
    }
    
    pub fn on_start(&mut self, _state : &mut GameState) {

        let mut ent_draft = self.clone();

        for i in 0..self.components.len() {

            self.components[i].on_start(&mut ent_draft, _state);
            ent_draft.components[i] = self.components[i].clone();

        }

        *self = ent_draft;

    }

    pub fn on_update(&mut self, _state : &mut GameState) {

        let mut ent_draft = self.clone();

        for i in 0..self.components.len() {

            self.components[i].on_update(&mut ent_draft, _state);
            ent_draft.components[i] = self.components[i].clone();

        }

        *self = ent_draft;

    }

    pub fn on_destroy(&mut self, _state : &mut GameState) {

        for i in 0..self.components.len() {

            self.components[i].on_destroy(_state);

        }
    }

    pub fn on_render(&self, _state : &mut GameState) {

        let transform = self.transform.clone();

        for i in 0..self.components.len() {

            self.components[i].render(transform, _state);

        }
        
    }

}

// Layer System

impl Entity {

    pub fn set_layer (&mut self, some_usize : usize) {
        self.layer = some_usize;
    }

    pub fn get_layer (&self) -> usize{

        return self.layer;

    }

}
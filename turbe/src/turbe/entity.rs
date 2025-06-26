use turbo::prelude::*;

use crate::{turbe, GameState};

use turbe::component_system::{component::Component, component_lifecycle::ComponentLifecycle};

use turbe::helpers;
use helpers::{transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Entity<T: ComponentLifecycle> {
    pub name: String,
    pub components: Vec<T>,
    pub transform: Transform,
    pub layer: usize,
    pub locat: usize,
}

impl<T: ComponentLifecycle> Entity<T> {

    pub fn new (name : String, vec : Vec<T>) -> Self {

        Self { 
            name: name, components: vec, transform: Transform::new(), layer: 0, locat: 0 
        }

    }

    pub fn new_base (_name : String) -> Entity<T> {

        return Entity::new(_name, vec![]);

    }

    pub fn add_component (&mut self, component : T) {

        self.components.push(component);

    }

}

// Lifetime Systems

impl<T: ComponentLifecycle> Entity<T> {

    pub fn on_init(&self, _state : &mut GameState) {

    }

    pub fn on_awake(&self, _state : &mut GameState) {
    
    }
    
    pub fn on_start(&self, _state : &mut GameState) {

    }

    pub fn on_update(&self, _state : &mut GameState) {

    }

    pub fn on_destroy(&self, _state : &mut GameState) {
        
        let mut ent_draft = self.clone();

        for i in 0..self.components.len() {

            self.components[i].on_update(&mut ent_draft, _state);
            ent_draft.components[i] = self.components[i].clone();

        }

        self = ent_draft;
    }

    pub fn on_render(&self, _state : &mut GameState) {

        let transform = self.transform.clone();

        for i in 0..self.components.len() {

            self.components[i].render(transform, _state);

        }
        
    }

}

// Layer System

impl<T: ComponentLifecycle> Entity<T> {

    pub fn set_layer (&mut self, some_usize : usize) {
        self.layer = some_usize;
    }

    pub fn get_layer (&self) -> usize{

        return self.layer;

    }

}
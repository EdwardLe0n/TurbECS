use turbo::*;

use crate::{turbe, GameState};

use turbe::component_system::{component::Component, component_types::ComponentTypes};

use turbe::helpers;
use helpers::{transform::Transform, active_states::ActiveStates};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct Entity {
    pub name: String,
    pub components: Vec<Component>,
    pub transform: Transform,
    pub layer: usize,
    pub locat: usize,
    pub state: ActiveStates
}

impl Entity {

    pub fn new (name : String, vec : Vec<Component>) -> Self {

        Self { 
            name: name, components: vec, transform: Transform::new(), layer: 0, locat: 0, state : ActiveStates::Inactive
        }

    }

    pub fn new_base (_name : String) -> Entity {

        return Entity::new(_name, vec![]);

    }

    pub fn add_component (&mut self, component : Component) {

        self.components.push(component);

    }

    pub fn find_component (&self, comp_type : ComponentTypes) -> usize {

        let mut i = 0;

        for j in 0..self.components.len() {

            if self.components[j].get_comp_type() == comp_type {
                return i;
            }

            i += 1;

        }

        return i;

    }

    pub fn check_component (&self, comp_type : ComponentTypes, locat : usize) -> bool {

        if self.components.len() <= locat {
            return false;
        }
        else if self.components[locat].get_comp_type() == comp_type {
            return true;
        }

        return false;

    }

    pub fn has_component (&self, comp_type : ComponentTypes) -> bool {

        return self.check_component(comp_type.clone(), self.find_component(comp_type));

    }

}

// Lifetime Systems

impl Entity {

    pub fn on_awake(&mut self, _state : &mut GameState) {

        if self.state == ActiveStates::Destroyed{
            return;
        }

        self.state = ActiveStates::Active;

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

        if self.state != ActiveStates::Active {
            return;
        }

        let mut ent_draft = self.clone();

        for i in 0..self.components.len() {

            self.components[i].on_update(&mut ent_draft, _state);
            ent_draft.components[i] = self.components[i].clone();

        }

        *self = ent_draft;

    }

    pub fn on_destroy(&mut self, _state : &mut GameState) {

        self.state = ActiveStates::Destroyed;

        for i in 0..self.components.len() {

            self.components[i].on_destroy(_state);

        }
    }

    pub fn on_render(&self, _state : &mut GameState) {

        if self.state != ActiveStates::Active {
            return;
        }

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
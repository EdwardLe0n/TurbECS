use turbo::*;
use crate::{turbecs, GameState};

use turbecs::component_system::{component::Component, component_types::ComponentTypes};

use turbecs::helpers;
use helpers::{transform::Transform, active_states::ActiveStates, has_x::HasX};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct Entity {
    pub name: String,
    pub components: Vec<Component>,
    pub transform: Transform,
    pub layer: usize,
    pub locat: usize,
    pub has: HasX,
    pub state: ActiveStates
}

impl Entity {

    pub fn new (name : String, vec : Vec<Component>) -> Self {

        Self { 
            name: name, components: vec, transform: Transform::new(), layer: 0, locat: 0, has: HasX::new(), state : ActiveStates::NtbAwake
        }

    }

    pub fn new_base (_name : String) -> Entity {

        return Entity::new(_name, vec![]);

    }

    pub fn add_component (&mut self, component : Component) {

        self.components.push(component);

    }

    pub fn find_component (&self, comp_type : ComponentTypes) -> (bool, usize) {

        let mut i = 0;

        for j in 0..self.components.len() {

            if self.components[j].get_comp_type() == comp_type {
                return (true, i);
            }

            i += 1;

        }

        return (false, i);

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

        return self.check_component(comp_type.clone(), self.find_component(comp_type).1);

    }

}

// Lifetime Systems

impl Entity {

    pub fn on_awake(&mut self, _state : &mut GameState) {

        // Sanity
        // log!("Trying {} with state {}", self.name, self.state.get_string());

        if self.state != ActiveStates::NtbAwake{
            return;
        }
        
        // Sanity
        // log!("Loading the {} entity!", self.name);

        self.state = ActiveStates::NtbStart;

        self.adjust_children();

        let len = self.components.len();

        for i in 0..len {

            let mut some_copy = self.components[i].clone();

            some_copy.on_awake(self, _state);

            self.components[i] = some_copy;

        }

        if _state.render_manager.len() <= self.layer
        {
            while _state.render_manager.len() <= self.get_layer() {
                _state.render_manager.push(Vec::new());
            }
        }

        _state.render_manager[self.get_layer()].push(self.locat);
    
    }
    
    pub fn on_start(&mut self, _state : &mut GameState) {

        if self.state != ActiveStates::NtbStart {
            return;
        }

        self.state = ActiveStates::Active;

        let len = self.components.len();

        for i in 0..len {

            let mut some_copy = self.components[i].clone();

            some_copy.on_start(self, _state);

            self.components[i] = some_copy;

        }

    }

    pub fn on_update(&mut self, _state : &mut GameState) {

        if self.state != ActiveStates::Active {
            return;
        }

        let len = self.components.len();

        for i in 0..len {

            let mut some_copy = self.components[i].clone();

            some_copy.on_update(self, _state);

            self.components[i] = some_copy;

        }

    }

    pub fn on_destroy(&mut self, _state : &mut GameState) {

        self.state = ActiveStates::Destroyed;

        for i in 0.._state.render_manager[self.get_layer()].len() {
            if self.locat == _state.render_manager[self.get_layer()][i] {


                // Sanity
                // log!("removing element with name {}", _state.entities[self.locat].name);

                _state.render_manager[self.get_layer()].remove(i);

                // Sanity
                // log!("Got rid fo a reference named {}!", self.name);

                break;
            }
        }

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

// Misc 
impl Entity {
    
    pub fn adjust_children(&mut self) {

        if !self.transform.position.get_bound_status() {
            return;
        }

    }

    pub fn is_active(&self) -> bool{

        if self.state == ActiveStates::Destroyed || self.state == ActiveStates::Inactive {
            return false;
        }

        return true;

    }

    pub fn is_destroyed(&self) -> bool{

        if self.state == ActiveStates::Destroyed {
            return true;
        }

        return false;

    }

}
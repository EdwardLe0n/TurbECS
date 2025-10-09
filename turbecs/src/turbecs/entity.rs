use turbo::*;
use crate::{turbecs, GameState};

use turbecs::component_system::{component::Component, component_types::ComponentTypes};

use turbecs::helpers;
use helpers::{transform::Transform, active_states::ActiveStates, has_x::HasX};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct Entity {
    pub name: String,
    pub comp_num: usize,
    pub comp_locats: Vec<usize>,
    pub transform: Transform,
    pub layer: usize,
    pub locat: usize,
    pub has: HasX,
    pub state: ActiveStates
}

impl Entity {

    pub fn new_base (name : String) -> Self {

        Self { 
            name: name, comp_num: 0, comp_locats: Vec::with_capacity(10), transform: Transform::new(), layer: 0, locat: 0, has: HasX::new(), state : ActiveStates::NtbAwake
        }

    }

    pub fn new (name : String, comp_vec : &mut Vec<Component>) -> Self {

        let mut temp = Self::new_base(name);
        temp.comp_num = comp_vec.len();

        return temp;

    }

    pub fn find_component (&self, comp_type : ComponentTypes, state : &mut GameState) -> (bool, usize) {

        let mut i = 0;

        for j in 0..self.comp_locats.len() {

            if state.component_manager.components[self.comp_locats[j]].get_comp_type() == comp_type {
                return (true, i);
            }

            i += 1;

        }

        return (false, i);

    }

    pub fn check_component (&self, comp_type : ComponentTypes, locat : usize, state : &mut GameState) -> bool {

        if self.comp_locats.len() <= locat {
            return false;
        }
        else if state.component_manager.components[self.comp_locats[locat]].get_comp_type() == comp_type {
            return true;
        }

        return false;

    }

    pub fn find_component_in_state (&self, comp_type : ComponentTypes, state : &mut GameState) -> (bool, usize) {

        let mut i = 0;

        for j in 0..self.comp_locats.len() {

            if state.component_manager.components[self.comp_locats[j]].get_comp_type() == comp_type {
                return (true, self.comp_locats[j]);
            }

            i += 1;

        }

        return (false, i);

    }

    pub fn has_component (&self, comp_type : ComponentTypes, state : &mut GameState) -> bool {

        return self.check_component(comp_type.clone(), self.find_component(comp_type, state).1, state);

    }

}

// Lifetime Systems

impl Entity {

    pub fn on_awake(&mut self, state : &mut GameState) {

        if self.state != ActiveStates::NtbAwake{
            return;
        }

        let len = self.comp_locats.len();

        for i in 0..len {

            if state.component_manager.components[self.comp_locats[i]].has.has_awake {
                let mut some_copy = state.component_manager.components[self.comp_locats[i]].clone();

                some_copy.on_awake(self, state);

                state.component_manager.components[self.comp_locats[i]] = some_copy;
            }

        }
    
    }
    
    pub fn on_start(&mut self, state : &mut GameState) {

        if self.state != ActiveStates::NtbStart {
            return;
        }

        let len = self.comp_locats.len();

        for i in 0..len {

            if state.component_manager.components[self.comp_locats[i]].has.has_start {
                let mut some_copy = state.component_manager.components[self.comp_locats[i]].clone();

                some_copy.on_start(self, state);

                state.component_manager.components[self.comp_locats[i]] = some_copy;
            }

        }

    }

    pub fn on_update(&mut self, state : &mut GameState) {

        if self.state != ActiveStates::Active {
            return;
        }

        let len = self.comp_locats.len();

        for i in 0..len {

            if state.component_manager.components[self.comp_locats[i]].has.has_update {
                let mut some_copy = state.component_manager.components[self.comp_locats[i]].clone();

                some_copy.on_update(self, state);

                state.component_manager.components[self.comp_locats[i]] = some_copy;
            }

        }

    }

    pub fn on_destroy(&mut self, state : &mut GameState) {

        let len = self.comp_locats.len();

        for i in 0..len {

            if state.component_manager.components[self.comp_locats[i]].has.has_destroy {
                let mut some_copy = state.component_manager.components[self.comp_locats[i]].clone();

                some_copy.on_destroy(state);

                state.component_manager.components[self.comp_locats[i]] = some_copy;
            }

        }
    }

    pub fn on_render(&self, state : &mut GameState) {

        if self.state != ActiveStates::Active {
            return;
        }

        let transform = self.transform.clone();

        for i in 0..self.comp_locats.len() {

            if state.component_manager.components[self.comp_locats[i]].has.has_render {

                let some_copy = state.component_manager.components[self.comp_locats[i]].clone();

                some_copy.on_render(transform, state);

            }

        }
        
    }

}

// Layer System

impl Entity {

    pub fn add_to_renderer(&mut self, state : &mut GameState) {

        if self.state != ActiveStates::NtbAwake{
            return;
        }

        if !self.has.has_render {
            return;
        }

        if state.render_manager.len() <= self.layer
        {
            while state.render_manager.len() <= self.get_layer() {
                state.render_manager.push(Vec::new());
            }
        }

        state.render_manager[self.get_layer()].push(self.locat);
    }

    pub fn remove_from_renderer(&mut self, state : &mut GameState) {

        if !self.has.has_render {
            return;
        }

        for i in 0..state.render_manager[self.get_layer()].len() {
            if self.locat == state.render_manager[self.get_layer()][i] {

                state.render_manager[self.get_layer()].remove(i);

                break;
            }
        }
    }

    pub fn set_layer (&mut self, some_usize : usize) {
        self.layer = some_usize;
    }

    pub fn get_layer (&self) -> usize{

        return self.layer;

    }

}

// Misc 
impl Entity {

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

    pub fn make_awoken(&mut self) {
        
        if self.state != ActiveStates::NtbAwake{
            return;
        }

        self.state = ActiveStates::NtbStart;
    }

    pub fn make_started(&mut self) {

        if self.state != ActiveStates::NtbStart {
            return;
        }

        self.state = ActiveStates::Active;
    }

    pub fn make_destroyed(&mut self) {
        self.state = ActiveStates::Destroyed;
    }

    pub fn handle_destroyed_components(&mut self, state : &mut GameState) {

        for comp in &self.comp_locats {
            state.component_manager.gap_data.empty_spaces.push_back(*comp);
        }

    }

}
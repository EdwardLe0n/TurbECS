use turbo::*;
use crate::{turbecs, GameState};

use turbecs::entity::Entity;
use turbecs::{component_system::component_types::ComponentTypes};

use turbecs::helpers;
use helpers::{gap_data::GapData, lifetime_data::LifetimeData};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct EntityManager {
    pub gap_data : GapData,
    pub lifetime_data : LifetimeData,
    pub entities : Vec<Entity>,
    pub new_entities : Vec<Entity>,
}

impl EntityManager {
    
    pub fn new() -> Self {

        return Self { 
            gap_data: GapData::new(), 
            lifetime_data: LifetimeData::new(), 
            entities: Vec::with_capacity(100), 
            new_entities: Vec::with_capacity(100) 
        };

    }

}

/*
* Lifetime functions
*/

/// Entity lifetime driver for all on_awake functionality

impl GameState {
    pub fn on_awake(&mut self) {

        let len = self.entity_manager.lifetime_data.new_awake.len();

        if len == 0 {
            return;
        }

        for _i in 0..len {

            let locat = *self.entity_manager.lifetime_data.new_awake.front().unwrap();

            let mut some_ent = self.entity_manager.entities[locat].clone();

            if self.entity_manager.entities[locat].has.has_awake {

                some_ent.on_awake(self);

            }

            some_ent.add_to_renderer(self);

            self.entity_manager.entities[locat] = some_ent;

            self.entity_manager.entities[locat].make_awoken();

            self.entity_manager.lifetime_data.new_start.push_back(*self.entity_manager.lifetime_data.new_awake.front().unwrap());
            self.entity_manager.lifetime_data.new_awake.pop_front();

        }

    }

    /// Entity lifetime driver for all on_start functionality

    pub fn on_start(&mut self) {

        let len = self.entity_manager.lifetime_data.new_start.len();

        if len == 0 {
            return;
        }

        for _i in 0..len {

            let locat = *self.entity_manager.lifetime_data.new_start.front().unwrap();

            if self.entity_manager.entities[locat].has.has_start {

                let mut some_ent = self.entity_manager.entities[locat].clone();

                some_ent.on_start(self);

                self.entity_manager.entities[locat] = some_ent;
            
            }

            self.entity_manager.entities[locat].make_started();

            self.entity_manager.lifetime_data.new_start.pop_front();

        }

    }

    /// Entity lifetime driver for all on_update functionality
    pub fn on_update(&mut self) {

        let len = self.entity_manager.entities.len();

        for i in 0..len {

            if !self.entity_manager.entities[i].is_active() {
                continue;
            }

            if !self.entity_manager.entities[i].has.has_update {
                continue;
            }

            let mut some_ent = self.entity_manager.entities[i].clone();

            some_ent.on_update(self);

            self.entity_manager.entities[i] = some_ent;

        }

        // Particle manager call to update particles
        // Code by Alex Feigenbaum

        self.particle_manager.update();

    }

    /// Entity lifetime driver for all on_destroy functionality

    pub fn on_destroy(&mut self) {

        let len = self.entity_manager.lifetime_data.new_destroy.len();

        if len == 0 {
            return;
        }

        for _i in 0..len {

            let locat = *self.entity_manager.lifetime_data.new_destroy.front().unwrap();

            if !self.entity_manager.entities[locat].is_destroyed() {

                self.entity_manager.entities[locat].make_destroyed();

                let mut some_ent = self.entity_manager.entities[locat].clone();

                some_ent.remove_from_renderer(self);

                if some_ent.has.has_destroy {
                    some_ent.on_destroy(self);
                }
                
                some_ent.handle_destroyed_components(self);

                self.entity_manager.entities[locat] = some_ent;

                self.entity_manager.gap_data.empty_spaces.push_back(locat);
                    
            }

            self.entity_manager.lifetime_data.new_destroy.pop_front();

        }

    }

    /// Entity lifetime driver for all on_render functionality

    pub fn on_render(&mut self) {

        clear(0xeeeeeeff);

        let render_layers = self.render_manager.len();

        for i in 0..render_layers {

            let render_elements = self.render_manager[i].len();

            for j in 0..render_elements{

                if !self.entity_manager.entities[self.render_manager[i][j]].has.has_render {
                    continue;
                }

                let some_ent = self.entity_manager.entities[self.render_manager[i][j]].clone();
                some_ent.on_render(self);

            }
        }

        self.particle_manager.draw();

    }
}


impl GameState {

    /// Pushes mutable entity to be instantiated at the next possible time
    pub fn new_entity(&mut self, _entity : &mut Entity) {

        self.entity_manager.new_entities.push(_entity.clone());

    }

    /// gets the number of available free spaces
    pub fn get_num_of_free_locat(&mut self) -> usize {
        return self.entity_manager.gap_data.empty_spaces.len();
    }

    /// Gets the next free available space, then pops the reference
    /// Assumes that there is is another available space
    pub fn get_next_free(&mut self) -> usize {

        let next = *self.entity_manager.gap_data.empty_spaces.front().unwrap();

        self.entity_manager.gap_data.empty_spaces.pop_front();

        return next;

    }
 
    /// Loops through the entire GameState to find a entity with a certain component
    /// If not found, the first element it returns will be false
    pub fn find_w_component(&mut self, some_type : ComponentTypes) -> (bool, usize) {

        for i in 0..self.entity_manager.entities.len() {

            if !self.entity_manager.entities[i].is_destroyed() {

                let ent =  self.entity_manager.entities[i].clone();

                if ent.has_component(some_type.clone(), self) {
                    return (true, i);
                }

            }

        }

        return (false, 0);

    }
}

/// Loading new entities functions
impl GameState {

    /// Loops through all the entities that need to be loaded,
    /// until everything has been loaded
    pub fn load_entities(&mut self) {

        if self.entity_manager.new_entities.len() == 0{
            return;
        }

        for ent in self.entity_manager.new_entities.clone() {
            self.load_entity(&mut ent.clone());
        } 

        self.entity_manager.new_entities.clear();

    }

    /// Loads the next immediate entity
    pub fn load_entity(&mut self, _entity : &mut Entity) {
        let mut next : usize = 0;

        if self.get_num_of_free_locat() > 0 {

            next = self.get_next_free();
            _entity.locat = next.clone();

            self.entity_manager.entities[next] = _entity.clone();

            // do linkage here!

        }
        else {

            next = self.entity_manager.entities.len();
            _entity.locat = next.clone();

            self.entity_manager.entities.push(_entity.clone());

            // do linkage here!

        }

        self.entity_manager.lifetime_data.new_awake.push_back(next);
    }
}
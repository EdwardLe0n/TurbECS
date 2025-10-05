use turbo::*;
use crate::{turbecs, GameState};

use turbecs::{entity::Entity, gap_data::GapData, lifetime_data::LifetimeData};


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
* Loading of new entities
*/

/*
* Lifetime functions
*/

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

    // Loops through all of the entities in the entity vector in order of newest to oldest
    pub fn on_update(&mut self) {

        let len = self.entity_manager.entities.len();

        for i in 0..len {

            if !self.entity_manager.entities[i].has.has_update {
                continue;
            }

            if !self.entity_manager.entities[i].is_active() {
                continue;
            }

            let mut some_ent = self.entity_manager.entities[i].clone();

            some_ent.on_update(self);

            self.entity_manager.entities[i] = some_ent;

        }

        // particles!!!!

        self.particle_manager.update();

    }

    pub fn on_destroy(&mut self) {

        let len = self.entity_manager.lifetime_data.new_destroy.len();

        if len == 0 {
            return;
        }

        for _i in 0..len {

            let locat = *self.entity_manager.lifetime_data.new_destroy.front().unwrap();

            if !self.entity_manager.entities[locat].is_destroyed() {

                self.entity_manager.entities[locat].make_destroyed();

                if self.entity_manager.entities[locat].has.has_destroy {

                    let mut some_ent = self.entity_manager.entities[locat].clone();
                    some_ent.on_destroy(self);
                    self.entity_manager.entities[locat] = some_ent;

                }

                self.entity_manager.gap_data.empty_spaces.push_back(locat);
                    
            }

            self.entity_manager.lifetime_data.new_destroy.pop_front();

        }

    }

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
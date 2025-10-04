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

        // Sanity
        // log!("Going to awake {} entities with a total of {} entities", len, self.entities.len());

        for _i in 0..len {

            // Sanity
            // log!("Awakening {} with name {}", *self.lifetime_data.new_awake.front().unwrap(), "test");

            let test_val = *self.entity_manager.lifetime_data.new_awake.front().unwrap();

            // Sanity
            // log!("{:?}", test_val);

            let mut some_ent = self.entity_manager.entities[test_val].clone();

            some_ent.on_awake(self);

            self.entity_manager.entities[test_val] = some_ent;

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

            let mut some_ent = self.entity_manager.entities[locat].clone();

            some_ent.on_start(self);

            self.entity_manager.entities[locat] = some_ent;

            self.entity_manager.lifetime_data.new_start.pop_front();

        }

    }

    // Loops through all of the entities in the entity vector in order of newest to oldest
    pub fn on_update(&mut self) {

        let len = self.entity_manager.entities.len();

        for i in 0..len {

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

            let mut some_ent = self.entity_manager.entities[locat].clone();

            if !some_ent.is_destroyed() {

                some_ent.on_destroy(self);
                self.entity_manager.entities[locat] = some_ent;

                self.entity_manager.gap_data.empty_spaces.push_back(locat);
                self.entity_manager.lifetime_data.new_destroy.pop_front();
                    
            }

        }

    }

    pub fn on_render(&mut self) {

        clear(0xeeeeeeff);

        let render_list = self.render_manager.clone();
        let entities = self.entity_manager.entities.clone();

        for i in 0..render_list.len() {

            for j in 0..render_list[i].len(){

                entities[render_list[i][j]].on_render(self);

            }
        }

        self.particle_manager.draw();

    }
}
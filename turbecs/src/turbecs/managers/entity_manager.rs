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
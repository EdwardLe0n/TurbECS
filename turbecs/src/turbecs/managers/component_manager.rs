use turbo::*;
use crate::{turbecs, GameState};

use turbecs::{component_system, gap_data::GapData, lifetime_data::LifetimeData};
use component_system::component::{Component};


#[turbo::serialize]
#[derive(PartialEq)]
pub struct ComponentManager {
    pub gap_data : GapData,
    pub lifetime_data : LifetimeData,
    pub components : Vec<Component>,
}

impl ComponentManager {

    pub fn new() -> Self {
        return Self{gap_data: GapData::new(), lifetime_data: LifetimeData::new(), components : Vec::with_capacity(300)};
    }

    pub fn new_component(&mut self, some_comp : Component) {

        

    }
    
}
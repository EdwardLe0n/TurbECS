use crate::{turbecs};

use turbecs::{component_system, helpers::gap_data::GapData};
use component_system::component::{Component};


#[turbo::serialize]
#[derive(PartialEq)]
pub struct ComponentManager {
    pub gap_data : GapData,
    pub components : Vec<Component>,
}

impl ComponentManager {

    /// Generates a new component manager with an initial capacity for 300 components
    pub fn new() -> Self {
        return Self{gap_data: GapData::new(), components : Vec::with_capacity(300)};
    }

    /// will return true if we can append the next component
    /// will return false if we'll replace another component
    pub fn next_comp_locat(&self) -> (bool, usize) {
        if self.gap_data.empty_spaces.len() > 0 {
            return (false, *self.gap_data.empty_spaces.front().unwrap());
        }
        else {
            return (true, self.components.len());
        }
    }

    /// Will either make a new component at the end of the vector,
    /// replace an existing component
    pub fn new_component(&mut self, some_comp : Component) {

        let locat = self.next_comp_locat();

        if locat.0 {
            self.components.push(some_comp);
        }
        else {
            self.components[locat.1] = some_comp;
            self.gap_data.empty_spaces.pop_front();
        }

    }
    
}
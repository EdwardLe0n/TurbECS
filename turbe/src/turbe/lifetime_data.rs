use std::collections::VecDeque;

use crate::{turbe, GameState};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct LifetimeData {
    pub new_awake : VecDeque<usize>,
    pub new_start : VecDeque<usize>,
    pub new_destroy : VecDeque<usize>
}

impl LifetimeData {
    
    pub fn new() -> LifetimeData {
        return LifetimeData{new_awake : VecDeque::new(), new_start : VecDeque::new(), new_destroy : VecDeque::new()};
    }

}
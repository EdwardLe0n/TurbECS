use std::collections::VecDeque;

use turbo::prelude::*;

use crate::{turbe, GameState};

use turbe::component_system::{component::Component};

use turbe::helpers;
use helpers::{transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
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
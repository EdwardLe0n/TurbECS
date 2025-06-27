use std::collections::VecDeque;

use turbo::prelude::*;

use crate::{turbe, GameState};

use turbe::component_system::{component::Component};

use turbe::helpers;
use helpers::{transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct GapData {
    pub empty_spaces : VecDeque<usize>,
    pub recently_filled : VecDeque<usize>
}

impl GapData {
    
    pub fn new() -> GapData {
        return GapData{empty_spaces : VecDeque::new(), recently_filled : VecDeque::new()};
    }

}
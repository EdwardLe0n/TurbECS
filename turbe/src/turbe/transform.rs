use turbo::prelude::*;

use super::{position, size};
use position::Position;
use size::Size;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Transform {
    pub position : Position,
    pub size : Size
}

impl Transform {

    pub fn new() -> Transform {

        return Transform{position : Position::new(), size : Size::new()};

    }
    
}
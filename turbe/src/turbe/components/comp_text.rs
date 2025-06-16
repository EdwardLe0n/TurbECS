use turbo::prelude::*;

use super::super::{component::Component, position::Position};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TextComponent {
    pub text : String,
    pub position : Position,
    color : u32,
    font : String
}

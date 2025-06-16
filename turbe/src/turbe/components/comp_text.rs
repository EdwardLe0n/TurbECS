use turbo::prelude::*;

use super::super::{component::Component, transform::Transform};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct TextComponent {
    pub text : String,
    pub transform : Transform,
    color : u32,
    font : String
}

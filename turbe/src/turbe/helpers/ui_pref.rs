use turbo::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]

pub enum Horizonontal {
    Left,
    Center,
    Right
}

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]

pub enum Vertical {
    Top,
    Center,
    Bottom
}
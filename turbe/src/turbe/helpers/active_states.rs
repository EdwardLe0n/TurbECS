use turbo::prelude::*;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]

pub enum ActiveStates {
    Active,
    Inactive,
    Destroyed
}
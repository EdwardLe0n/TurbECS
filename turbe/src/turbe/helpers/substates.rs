use turbo::prelude::*;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]

pub enum SubStates {
    None,
    Enter,
    Persist,
    Exit
}
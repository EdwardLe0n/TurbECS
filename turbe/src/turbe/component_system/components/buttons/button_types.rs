use turbo::prelude::*;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum ButtonTypes {
    Default,
    Test,

    // User made buttons
    Title,
    Title2

}
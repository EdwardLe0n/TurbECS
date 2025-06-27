use turbo::prelude::*;

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum ComponentTypes {
    Camera,
    Rectangle,
    Sprite,
    Text,
    Button,

    // User made structs

    Increment,
    Move,
    Orbit,

    // Other

    Other

}
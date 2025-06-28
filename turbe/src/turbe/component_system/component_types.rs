use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]
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
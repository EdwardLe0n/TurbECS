use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]

pub enum SubStates {
    None,
    Enter,
    Persist,
    Exit
}
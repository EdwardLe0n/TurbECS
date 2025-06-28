use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]

pub enum ActiveStates {
    Active,
    Inactive,
    Destroyed
}
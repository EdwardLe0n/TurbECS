use turbo::*;

#[turbo::serialize]
#[derive(Copy, PartialEq)]

pub enum Horizonontal {
    Left,
    Center,
    Right
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]

pub enum Vertical {
    Top,
    Center,
    Bottom
}
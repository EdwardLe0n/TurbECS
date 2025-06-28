use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ButtonTypes {
    Default,
    Test,

    // User made buttons
    Title,
    Title2

}
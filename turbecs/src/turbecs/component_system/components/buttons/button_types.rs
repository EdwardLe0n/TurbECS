use crate::GameState;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ButtonTypes {
    Default,

    Title,
    Misc,
    Test,

    // User made buttons

}

impl ButtonTypes {

    pub fn can_still_interact(&self, _state : &mut GameState) -> bool {

        match self {
            _default => {return false;}
        }

    }

}
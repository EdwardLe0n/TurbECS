use turbo::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Flip {
    pub x : bool,
    pub y : bool,
}

impl Flip {

    pub fn new() -> Flip {

        return Flip { x : false , y : false }

    }

}

impl Flip {

    pub fn get_x(&self) -> bool {
        return self.x;
    }

    pub fn get_y(&self) -> bool {
        return self.y;
    }

    pub fn set_x(&mut self, some_bool : bool){
        self.x = some_bool;
    }

    pub fn set_y(&mut self, some_bool : bool){
        self.y = some_bool;
    }

}


// Add flip calls later
impl Flip {
    


}
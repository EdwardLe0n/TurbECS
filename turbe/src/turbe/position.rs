use turbo::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Position {
    pub x : i32,
    pub y : i32,
    pub rotation : i32,
}

impl Position {

    pub fn new() -> Position {

        return Position { x: 0, y: 0, rotation: 0 }

    }

    pub fn new_with_xy(some_x : i32, some_y : i32) -> Position {

        let mut transform = Position::new();

        transform.set_x(some_x);
        transform.set_y(some_y);

        return transform;

    }

}

impl Position {

    pub fn set_x(&mut self, some_x : i32) {
        self.x = some_x;
    }

    pub fn set_y(&mut self, some_y : i32) {
        self.y = some_y;
    }

    pub fn set_rotation(&mut self, some_rotation : i32) {
        self.rotation = some_rotation;
    }
    
    pub fn get_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_y(&self) -> i32 {
        return self.y;
    }

    pub fn get_rotation(&self) -> i32 {
        return self.rotation;
    }

}

impl Position {
    
    pub fn nudge_x(&mut self, some_x : i32) {

        self.set_x(self.get_x() + some_x);

    }

    pub fn nudge_y(&mut self, some_y : i32) {

        self.set_y(self.get_y() + some_y);

    }

    pub fn nudge_rotation(&mut self, some_rotation : i32) {

        self.set_rotation(self.get_rotation() + some_rotation);

    }

}
use super::size::Size;

#[turbo::serialize]
#[derive(Copy, PartialEq)]

pub struct BoundData {
    
    pub is_ui : bool,
    pub size : Size,
    pub horizontal : Horizontal,
    pub vertical : Vertical

}

impl BoundData {
    
    pub fn new() -> BoundData {
        return BoundData{is_ui : false, size : Size::new(), horizontal : Horizontal::Center, vertical : Vertical::Center};
    }

}

impl BoundData {
    
    pub fn set_ui_status(&mut self, some_bool : bool) {
        self.is_ui = some_bool;
    }

    pub fn set_horizontal_pref(&mut self, some_pref : Horizontal) {
        self.horizontal = some_pref;
    }

    pub fn set_vertical_pref(&mut self, some_pref : Vertical) {
        self.vertical = some_pref;
    }

    pub fn get_ui_status(&self) -> bool {
        return self.is_ui;
    }

    pub fn get_horizontal_pref(&self) -> Horizontal {
        return self.horizontal;
    }

    pub fn get_vertical_pref(&self) -> Vertical {
        return self.vertical;
    }

}

#[turbo::serialize]
#[derive(Copy, PartialEq)]

pub enum Horizontal {
    Left,
    Center,
    Right
}

impl Horizontal {
    pub fn get_string(&self) -> String {

        match self {
            Horizontal::Left => {return "left".to_string();},
            Horizontal::Center => {return "center".to_string();},
            Horizontal::Right => {return "right".to_string();},
        }

    }
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]

pub enum Vertical {
    Top,
    Center,
    Bottom
}
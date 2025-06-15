use turbo::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Size {
    pub width : i32,
    pub height : i32,
    pub scale_x : f32,
    pub scale_y : f32
}

impl Size {

    pub fn new() -> Size {

        return Size { width: 0, height: 0, scale_x : 1.0 , scale_y : 1.0 }

    }

    pub fn new_with_wh(some_width : i32, some_height : i32) -> Size {

        let mut size = Size::new();

        size.set_width(some_width);
        size.set_height(some_height);

        return size;

    }

}

impl Size {

    pub fn set_width(&mut self, some_width : i32) {
        self.width = some_width;
    }

    pub fn set_height(&mut self, some_height : i32) {
        self.height = some_height;
    }

    pub fn set_scale_x(&mut self, some_scale_x : f32) {
        self.scale_x = some_scale_x;
    }

    pub fn set_scale_y(&mut self, some_scale_y : f32) {
        self.scale_y = some_scale_y;
    }
    
    pub fn get_width(&self) -> i32 {
        return self.width;
    }

    pub fn get_height(&self) -> i32 {
        return self.height;
    }

    pub fn get_scale_x(&self) -> f32 {
        return self.scale_x;
    }

    pub fn get_scale_y(&self) -> f32 {
        return self.scale_y;
    }

}
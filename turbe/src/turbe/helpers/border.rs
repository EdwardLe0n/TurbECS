use turbo::prelude::*;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Border {

    size : u32,
    color : u32,
    radius : u32

}

impl Border {
    
    pub fn new () -> Border{
        return Border{size : 0, color : 0xffffffff, radius : 0};
    }

}

impl Border {
    
    pub fn set_size(&mut self, some_size : u32) {
        self.size = some_size;
    } 

    pub fn set_color(&mut self, some_color : u32) {
        self.color = some_color;
    }

    pub fn set_radius(&mut self, some_radius : u32) {
        self.radius = some_radius;
    }

    pub fn get_size(&self) -> u32 {
        return self.size;
    }

    pub fn get_color(&self) -> u32 {
        return self.color;
    }

    pub fn get_radius(&self) -> u32 {
        return self.radius;
    }

}
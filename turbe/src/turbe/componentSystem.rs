// Necessary imports

use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;

use super::componentTypes::ComponentTypes;
use super::targetSystem::Target;

// Base model????
#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Component {

    pub comp_type: ComponentTypes,
    pub comp_locat: (usize, usize),
    pub i32_vec : Vec<i32>,
    pub u32_vec : Vec<u32>,
    pub f32_vec : Vec<f32>,
    pub str_vec : Vec<String>,
    pub bool_vec : Vec<bool>,
    pub target_vec : Vec<Target>,
    pub ref_by : Vec<Target>

}


/* General Calls */

impl Component {

    // Allows for the first two elements of a component to always be positional based!
    pub fn init_position() -> (Vec<i32>) {

        let mut temp_vec : Vec<i32> = Vec::new();

        temp_vec.push(0);
        temp_vec.push(0);

        return temp_vec;

    }

    pub fn get_x(&self) -> i32 {
        return self.i32_vec[0];
    }

    pub fn get_center_x(&self) -> i32 {
        return self.get_x() - (self.get_width() as i32)/2;
    }

    pub fn get_y(&self) -> i32 {
        return self.i32_vec[1];
    }

    pub fn get_center_y(&self) -> i32 {
        return self.get_y() - (self.get_height() as i32)/2;
    }

    pub fn set_x_y(&mut self, locat:(i32, i32)) {
        self.i32_vec[0] = locat.0;
        self.i32_vec[1] = locat.1;
    }

    // Sets the first value in the bool vec to true, so that the component is viewed as active!
    pub fn init_hidden() -> Vec<bool> {

        let mut temp_vec : Vec<bool> = Vec::new();

        temp_vec.push(false);

        return temp_vec;

    }

    pub fn init_ui_element_status(&mut self) {
        self.bool_vec.push(false);
    }

    pub fn set_ui_element_status(&mut self, _some_bool : bool) {
        self.bool_vec[1] = _some_bool;
    }

    pub fn get_ui_element_status(& self) -> bool {
        return self.bool_vec[1];
    }

    pub fn init_center_status(&mut self) {
        self.bool_vec.push(false);
    }

    pub fn set_center_status(&mut self, _some_bool : bool) {
        self.bool_vec[2] = _some_bool;
    }

    pub fn get_center_status(&self) -> bool {
        return self.bool_vec[2];
    }

    pub fn init_color(&mut self) {
        self.u32_vec.push(0);
    }

    pub fn set_color(&mut self, _some_color : u32) {
        self.u32_vec[0] = _some_color;
    }

    pub fn get_color(&self) -> u32 {
        return self.u32_vec[0];
    } 

    pub fn init_width_height(&mut self, _some_width : u32, _some_height : u32) {
        self.u32_vec.push(_some_width);
        self.u32_vec.push(_some_height);
    }

    pub fn get_width(&self) -> u32 {
        return self.u32_vec[1];
    }

    pub fn get_height(&self) -> u32 {
        return self.u32_vec[2];
    }

    pub fn init_rotation(&mut self) {
        self.i32_vec.push(0);
    }

    pub fn set_rotation(&mut self, _some_i32 : i32) {
        self.i32_vec[2] = _some_i32;
    }

    pub fn get_rotation(&self) -> i32 {
        return self.i32_vec[2];
    }

    pub fn init_border_vals(&mut self) {
        self.i32_vec.push(0);
        self.u32_vec.push(0x00000000);
    }

    pub fn get_border_size(&self) -> i32 {
        return self.i32_vec[3];
    }

    pub fn get_border_color(&self) -> u32 {
        return self.u32_vec[3];
    } 

    pub fn new(comp_type : ComponentTypes) -> Self {

        Self {
            comp_type : comp_type,
            i32_vec : Component::init_position(),
            comp_locat : (0, 0),
            u32_vec : Vec::new(),
            f32_vec : Vec::new(),
            str_vec : Vec::new(),
            bool_vec : Component::init_hidden(),
            target_vec : Vec::new(),
            ref_by : Vec::new()
        }
        
    }

    pub fn change_component_offset(&mut self, _some_x : i32, _some_y : i32) {
        self.i32_vec[0] = _some_x;
        self.i32_vec[1] = _some_y;
    }

}

/*
    Lifetime functions
*/

impl Component {

    pub fn on_init(&mut self, some_vec : Vec<usize>) {

        self.comp_locat.0 = some_vec[0];
        self.comp_locat.1 = some_vec[1];

        // Sanity
        log!("{:?} component is at {:?}", self.comp_type.get_comp_type_str(), self.get_debug_locat());

    }

    pub fn on_awake(&mut self) {
        
        match self.comp_type {
            _ => {}
        }
        
    }

    pub fn on_start(&mut self) {
        match self.comp_type {
            _ => {}
        }
    }

    pub fn on_update(&mut self) {

        if !self.bool_vec[0] {
            match self.comp_type {
                ComponentTypes::Rectangle => {
                    self.rotate_rect();
                }
                ComponentTypes::CameraController => {
                    self.update_cam_pos();
                }
                _ => {}
            }
        }
    }

    pub fn on_destroy(&mut self) {
        match self.comp_type {
            _ => {}
        }
    }

    pub fn render(&mut self, _x : i32, _y : i32, _cam_x : i32, _cam_y : i32) {

        if !self.bool_vec[0] {

            match self.comp_type {
                ComponentTypes::Text => {
                    self.render_text(_x, _y, _cam_x, _cam_y);
                }
                ComponentTypes::Rectangle => {
                    self.render_rect(_x, _y, _cam_x, _cam_y);
                }
                _ => {}
            }

        }

    }
}

/*
    Debug functions
*/

impl Component {

    pub fn get_debug_locat(&mut self) -> String
    {

        let mut temp_str : String = "".to_string();

        temp_str.insert_str(temp_str.len(), &self.comp_locat.0.to_string());
        temp_str.insert_str(temp_str.len(), ", ");
        temp_str.insert_str(temp_str.len(), &self.comp_locat.1.to_string());

        return temp_str;

    }

}
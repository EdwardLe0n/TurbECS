// Necessary imports

use crate::kitchen::componentSystem::Component;
use crate::kitchen::componentTypes::ComponentTypes;
use crate::GameState;
use turbo::prelude::*;

/*
    Rectangle Implementation
*/

impl Component {

    pub fn init_border_radius(&mut self) {
        self.u32_vec.push(0);
    }

    pub fn get_border_radius(&self) -> u32 {
        return self.u32_vec[4];
    } 

    pub fn new_square(_some_len : u32) -> Component {
        return Component::new_rectangle(_some_len, _some_len);
    }

    pub fn new_rectangle(_some_width : u32 , _some_height : u32) -> Component {
        
        let mut some_component : Component = Component::new(ComponentTypes::Rectangle);

        some_component.init_ui_element_status();
        some_component.init_center_status();

        // needs to init color first!
        some_component.init_color();

        // Sets some base postition stuff
        some_component.init_width_height(_some_width, _some_height);
        some_component.init_rotation();
        some_component.init_border_vals();
        some_component.init_border_radius();

        return some_component;

    }

    pub fn rotate_rect(&mut self) {

        // let mut state = GameState::load();

        // state.current_game_objects[self.comp_locat.0]
        //     .components[self.comp_locat.1]
        //     .set_rotation(self.get_rotation() + 7);

        // state.save();

        self.set_rotation(self.get_rotation() + 7);

    }

    pub fn render_rect(&self, _x : i32, _y : i32, _cam_x : i32, _cam_y : i32) {

        if self.get_ui_element_status() {

            if self.get_center_status() {
                rect!(
                    w = self.get_width(),
                    h = self.get_height(),
                    x = _cam_x + self.get_center_x(),
                    y = _cam_y + self.get_center_y(),
                    color = self.get_color(),
                    rotation = self.get_rotation(),
                    border_size = self.get_border_size().try_into().unwrap(),
                    border_color = self.get_border_color(),
                    border_radius = self.get_border_radius(),
                )
            }
            else {
                rect!(
                    w = self.get_width(),
                    h = self.get_height(),
                    x = _cam_x + self.get_x(),
                    y = _cam_y + self.get_y(),
                    color = self.get_color(),
                    rotation = self.get_rotation(),
                    border_size = self.get_border_size().try_into().unwrap(),
                    border_color = self.get_border_color(),
                    border_radius = self.get_border_radius(),
                )
            }
        }
        else {

            if self.get_center_status() {
                rect!(
                    w = self.get_width(),
                    h = self.get_height(),
                    x = _x + self.get_center_x(),
                    y = _y + self.get_center_y(),
                    color = self.get_color(),
                    rotation = self.get_rotation(),
                    border_size = self.get_border_size().try_into().unwrap(),
                    border_color = self.get_border_color(),
                    border_radius = self.get_border_radius(),
                )
            }
            else {
                rect!(
                    w = self.get_width(),
                    h = self.get_height(),
                    x = _x + self.get_x(),
                    y = _y + self.get_y(),
                    color = self.get_color(),
                    rotation = self.get_rotation(),
                    border_size = self.get_border_size().try_into().unwrap(),
                    border_color = self.get_border_color(),
                    border_radius = self.get_border_radius(),
                )
            }
        }
    }

}
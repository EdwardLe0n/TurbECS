// Necessary imports

use turbo::prelude::*;

use crate::kitchen::componentSystem::Component;
use crate::kitchen::componentTypes::ComponentTypes;
use crate::GameState;

/*
    Text Implementation
*/

impl Component {

    
    /*
        The text portion
     */

     pub fn new_text(desired_text : String, ui_element: bool) -> Component {

        let mut some_component = Component::new(ComponentTypes::Text);

        some_component.str_vec.push(desired_text);
        some_component.init_ui_element_status();
        some_component.set_ui_element_status(ui_element);
        
        // Default values for color and text size
        some_component.init_color();
        some_component.str_vec.push("small".to_string());

        // Then set the color so a white to be viewable
        some_component.set_color(0xffffffff);

        // Returns the component ref in the tail
        return some_component;
    }

    pub fn change_text_size(&mut self, some_str : String) {

        // Sanity

        // log!("The value prior to the change is... {:?}", self.str_vec.get(1).unwrap());
        // log!("The value attempting to get inserted is... {:?}", some_str);

        self.str_vec[1] = some_str;
        
        // log!("The value after the change is... {:?}", self.str_vec.get(1).unwrap());

    }

    pub fn set_font(&mut self, some_str : String) {
        
        let mut state = GameState::load();

        state.current_game_objects[self.comp_locat.0].components[self.comp_locat.1].str_vec[1] = some_str;

        state.save();

    }

    pub fn get_font(&self) -> &str {
        return &self.str_vec[1];
    }

    pub fn change_text_str(&mut self, some_str : String) {
        self.str_vec[0] = some_str;
    }

    pub fn render_text(&self, _x : i32, _y : i32, _cam_x : i32, _cam_y : i32)
    {
        if self.get_ui_element_status()
        {
            text!(
                self.str_vec.get(0).unwrap(), 
                x = _cam_x + self.get_x(),
                y = _cam_y + self.get_y(),
                color = self.get_color(),
                font = self.get_font())
        }
        else {
            text!(
                self.str_vec.get(0).unwrap(),
                x = _x + self.get_x(),
                y = _y + self.get_y(),
                color = self.get_color(),
                font = self.get_font())
        }
    }

}
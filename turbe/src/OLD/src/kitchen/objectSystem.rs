// Necessary imports

use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;

// Imports to other files

use crate::kitchen::componentSystem::Component;
use crate::kitchen::targetSystem;
use crate::GameState;

use super::targetSystem::Target;

/*
    The GameObject System!

    Basic system to utilize a game object that has multiple components under it!

*/

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct GameObject {

    pub(super) name : String,
    pub(super) components : Vec<Component>,
    pub(super) x : i32,
    pub(super) y : i32,
    pub(super) layer : u8,
    pub(super) tag : String,
    pub(super) parent : bool,
    pub(super) locat : Vec<usize>,
    pub(super) ref_by : Vec<Target>
    
}

/*

    Setting up game object functions!

*/

impl GameObject {

    pub fn new(_name : String) -> GameObject  {
        let temp : GameObject = GameObject {
            name : _name,
            components : Vec::new(),
            x : 0,
            y : 0,
            layer : 0, 
            tag : "default".to_string(),
            parent : true,
            locat : Vec::new(),
            ref_by : Vec::new()
        };

        return temp;
    }

    pub fn new_with_components(_name : String, components : Vec<Component>) -> Self  {
        let temp : GameObject = GameObject {
            name : _name,
            components : components,
            x : 0,
            y : 0,
            layer : 0, 
            tag : "default".to_string(),
            parent : true,
            locat : Vec::new(),
            ref_by : Vec::new()
        };

        return temp;
    }

    pub fn new_camera() -> GameObject {

        let temp_comp : Component = Component::new_camera();
        let mut temp_vec : Vec<Component> = Vec::new();
        temp_vec.push(temp_comp);

        return GameObject::new_with_components("Camera".to_string(), temp_vec);

    }

}

/*

    Setters and getters!

*/

impl GameObject {

    pub fn set_layer(&mut self, some_layer : u8)
    {
        self.layer = some_layer;
    }

    /*
        Setters & Getters
     */

    pub fn get_name(&self) -> String {
        return self.name.clone();
    }

    pub fn get_pos_x(&self) -> i32 {
        return self.x;
    }

    pub fn get_pos_y(&self) -> i32 {
        return self.y;
    }

    pub fn get_layer(&self) -> u8 {
        return self.layer;
    }

    /*

        Updating Functions...

     */

    pub fn add_component(&mut self, newComponent : Component)
    {
        self.components.push(newComponent);
    }

    pub fn update_location(&mut self, _some_x : i32, _some_y : i32)
    {
        self.x = _some_x;
        self.y = _some_y;
    }

    pub fn nudge_position(&mut self, some_x : i32, some_y : i32)
    {
        self.x += some_x;
        self.y += some_y;
    }

}

/*

    Lifetime functions!

*/

impl GameObject {

    pub fn on_init(&mut self, parent_usize : usize, some_usize : usize)
    {

        let mut state = GameState::load();

        self.locat.insert(0, parent_usize);
        self.locat.insert(1, some_usize);

        let mut temp_vec : Vec<usize> = Vec::new();
        temp_vec.push(some_usize);

        state.save();

        for i in 0..self.components.len() {

            let mut other_temp_vec : Vec<usize> = temp_vec.clone();
            other_temp_vec.push(i);

            self.components[i].on_init(other_temp_vec);

        }

        state = GameState::load();

        if(self.locat[0] == self.locat[1]) {
            self.parent = true;

            while state.current_parent_objects.len() < usize::from(self.layer) + 1
            {

                state.current_parent_objects.push(Vec::new());
                log!("Making a new layer {:?}", (state.current_parent_objects.len() - 1));

            }

            state.current_parent_objects[usize::from(self.layer)].push(self.locat[1]);

        }

        state.save();

        // Also fixes up parent object

    }

    pub fn on_awake(&mut self)
    {

        for i in 0..self.components.len() {
            self.components[i].on_awake();
        }
        
    }
    
    pub fn on_start(&mut self)
    {
        for i in 0..self.components.len() {
            self.components[i].on_start();
        }
    }

    pub fn on_update(&mut self)
    {

        for i in 0..self.components.len() {

            self.components[i].on_update();
        }

        // state.save();
    }

    pub fn on_destroy(&mut self)
    {
        for i in 0..self.components.len() {
            self.components[i].on_destroy();
        }
    }

    pub fn on_render(&mut self, camX : i32, camY : i32)
    {
        
        for i in 0..self.components.len() {
            self.components[i].render(self.x, self.y, camX, camY);
        }

    }
}

/*
    Debug functions
*/

impl GameObject {

    pub fn get_debug_locat(&mut self) -> String
    {

        let mut temp_str : String = "".to_string();

        for i in 0..self.locat.len() {
            
            temp_str.insert_str(temp_str.len(), 
                        self.locat.get(i).unwrap().to_string().as_str());

            temp_str.insert_str(temp_str.len(), ", ");

        }

        return temp_str;

    }

}
use borsh::{BorshDeserialize, BorshSerialize};
use turbo::prelude::*;

#[derive(Debug, Copy,Clone, PartialEq, BorshSerialize, BorshDeserialize)]

pub enum ComponentTypes {
    None,
    Camera,
    Text,
    Rectangle,
    Error,

    // User made components!!!

    CameraController

}

impl ComponentTypes {
    
    pub fn get_comp_type_str(self) -> String {

        match self {

            ComponentTypes::Camera => {return "Camera".to_string()}
            ComponentTypes::Text => {return "Text".to_string()}
            ComponentTypes::Rectangle => {return "Rectangle".to_string()} 
            
            ComponentTypes::Error => {return "disconnected object".to_string();}

            _ => {return "non-existent ".to_string()}
            
        }

    }

}
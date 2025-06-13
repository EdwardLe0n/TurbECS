// Necessary imports

use turbo::prelude::*;

use crate::kitchen::componentSystem::Component;
use crate::kitchen::componentTypes::ComponentTypes;

/*
    Camera implementation
*/

impl Component {

    pub fn new_camera() -> Component {

        return Component::new(ComponentTypes::Camera);

    }
    
}
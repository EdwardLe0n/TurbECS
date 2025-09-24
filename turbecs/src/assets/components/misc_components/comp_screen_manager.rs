use turbo::*;

use crate::{GameState};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct ScreenManagerComponent {

    pub moving : bool,
    pub cam_tween : Tween<f32>

}

impl ScreenManagerComponent {
    
    pub fn new() -> Self {

        return Self{moving : false, cam_tween : Tween::new(0.0).duration(60).ease(Easing::EaseInOutCirc).set(0.0)};

    }

}

impl ScreenManagerComponent {
    
    pub fn update(&mut self, state : &mut GameState) {

        if self.moving {
            
            camera::set_x(self.cam_tween.get());

            if self.cam_tween.done() {

                self.moving = false;
                state.can_interact = true;

            }

        }

    }

}

impl ScreenManagerComponent {
    
    pub fn start_swipe(&mut self, state : &mut GameState, right : bool) -> bool {

        if self.moving {
            return false;
        }

        self.moving = true;

        // Turns off the ability to interact with other ui elements

        state.can_interact = false;

        self.cam_tween = Tween::new(camera::x()).duration(30).ease(Easing::EaseInOutCirc);

        if right {
            self.cam_tween.set(camera::x() + screen().w() as f32);
        }
        else {
            self.cam_tween.set(camera::x() - screen().w() as f32);
        }

        return true;

    }

}
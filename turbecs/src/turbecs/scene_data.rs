use turbo::*;

use std::collections::VecDeque;

use crate::{turbecs};
use turbecs::entity::Entity;

use crate::assets;

use assets::prefabs::{general_prefabs};

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct SceneData {
    pub active_scene : Scenes,
    pub is_loaded : bool
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum Scenes {
    Misc
}

impl SceneData {
    pub fn load_scene(&mut self, _some_scene : Scenes) {

        self.active_scene = _some_scene;
        self.is_loaded = false;

    }
}

pub fn make_scene (some_scene : Scenes) ->  VecDeque<Entity>{

    match some_scene {
        Scenes::Misc => {return make_misc_scene()},
        _default => {
            return VecDeque::new();
        }
    }

}

pub fn apply_screen_offset(some_screen_vec : &mut VecDeque<Entity>, some_x : i32) {

    for ent in some_screen_vec {

        ent.transform.nudge_x(some_x);

    }

}

pub fn make_misc_scene() -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();


    return ent_vec;

}
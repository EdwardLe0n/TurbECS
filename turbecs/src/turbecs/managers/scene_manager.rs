use turbo::*;

use std::collections::VecDeque;

use crate::{turbecs};
use turbecs::entity::Entity;
use turbecs::component_system::component::Component;

use crate::assets;

use assets::prefabs::{general_prefabs};

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct SceneManager {
    pub active_scene : Scenes,
    pub is_loaded : bool
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum Scenes {
    Title,
    Misc
}

impl SceneManager {

    pub fn new() -> Self {

        return Self{active_scene : Scenes::Title, is_loaded : false};

    }

    pub fn load_scene(&mut self, _some_scene : Scenes) {

        self.active_scene = _some_scene;
        self.is_loaded = false;

    }
}

pub fn make_scene (some_scene : Scenes) ->  VecDeque<(Entity, VecDeque<Component>)>{

    match some_scene {
        Scenes::Title => {return make_title_scene()},
        Scenes::Misc => {return make_misc_scene()},
        _default => {
            return VecDeque::new();
        }
    }

}

pub fn make_title_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_title());

    ent_vec.push_back(general_prefabs::new_to_misc());

    return ent_vec;

}

pub fn make_misc_scene() -> VecDeque<(Entity, VecDeque<Component>)> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_to_title());

    return ent_vec;

}
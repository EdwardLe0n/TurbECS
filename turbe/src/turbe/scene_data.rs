use turbo::*;
use std::collections::VecDeque;

use crate::{turbe, GameState};
use turbe::component_system::component::Component;
use turbe::entity::Entity;

use crate::assets::prefabs;

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub struct SceneData {
    pub active_scene : Scenes,
    pub is_loaded : bool
}

#[turbo::serialize]
#[derive(Copy, PartialEq)]
pub enum Scenes {
    Title,
    Title2,
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
        Scenes::Title => {make_title_scene()},
        Scenes::Title2 => {make_title_2_scene()},
        _default => {
            return VecDeque::new();
        }
    }

}

pub fn make_title_scene () -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_front(prefabs::new_spr());
    ent_vec.push_front(prefabs::new_rect());
    ent_vec.push_front(prefabs::new_text());

    ent_vec.push_front(prefabs::new_button());

    ent_vec.push_front(prefabs::new_to_title_2());

    return ent_vec;

}

pub fn make_title_2_scene () -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_front(prefabs::new_to_title());
    
    return ent_vec;

}
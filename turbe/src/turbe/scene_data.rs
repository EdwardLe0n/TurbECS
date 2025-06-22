use turbo::prelude::*;
use std::collections::VecDeque;

use crate::turbe;
use turbe::component::{Component};
use turbe::entity::Entity;

use crate::assets::prefabs;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SceneData {
    pub active_scene : Scenes,
    pub is_loaded : bool
}

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Scenes {
    Title,
    Misc
}

pub fn make_scene (some_scene : Scenes) ->  VecDeque<Entity<Component>>{

    match some_scene {
        Scenes::Title => {make_title_scene()},
        _default => {
            return VecDeque::new();
        }
    }

}

pub fn make_title_scene () -> VecDeque<Entity<Component>> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_front(prefabs::new_spr());
    ent_vec.push_front(prefabs::new_rect());
    ent_vec.push_front(prefabs::new_text());

    return ent_vec;

}
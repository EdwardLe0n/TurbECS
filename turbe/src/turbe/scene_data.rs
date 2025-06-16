use turbo::prelude::*;
use std::collections::VecDeque;

use crate::turbe::component::{Component, ComponentLifecycle};
use crate::assets::prefabs;

use super::entity::Entity;

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct SceneData {
    pub active_scene : Scenes,
    pub is_loaded : bool
}

#[derive(Debug, Copy, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Scenes {
    Title
}

pub fn make_scene (some_scene : Scenes) ->  VecDeque<Entity<Component>>{

    match some_scene {
        Scenes::Title => {make_title_scene()},
        default => {
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
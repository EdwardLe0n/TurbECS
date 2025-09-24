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
    Title,
    Intro,
    DeckSelect,
    Ready,
    Battle,
    Shop,
    LiveFeed,
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
        Scenes::Title => {return make_title_scene()},
        Scenes::Ready => {return make_ready_scene()},
        Scenes::Battle => {return make_battle_scene()},
        Scenes::Misc => {return make_misc_scene()},
        Scenes::DeckSelect => {return make_deck_list()},
        Scenes::Shop => {return make_shop_scene();},
        Scenes::Intro => {return make_intro_scene();},
        Scenes::LiveFeed => {return make_live_feed_scene()},
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

pub fn make_title_scene () -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_front(general_prefabs::new_screen_manager());

    // Home screen

    ent_vec.push_front(general_prefabs::new_title());
    ent_vec.push_front(general_prefabs::new_to_play());
    ent_vec.push_front(general_prefabs::new_to_options());
    ent_vec.push_front(general_prefabs::new_main_quit());

    // ent_vec.push_front(general_prefabs::new_to_live_feed());

    ent_vec.push_front(general_prefabs::new_swipe_right());
    ent_vec.push_front(general_prefabs::new_ver_notice());

    ent_vec.push_front(general_prefabs::new_reset());

    // Solely for testing
    //ent_vec.push_front(general_prefabs::new_to_misc());
    
    // player info screen

    let mut player_info_screen = VecDeque::new();

    player_info_screen.push_front(general_prefabs::new_swipe_left());

    apply_screen_offset(&mut player_info_screen, screen().w() as i32);

    ent_vec.append(&mut player_info_screen);

    return ent_vec;

}

pub fn make_intro_scene () -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_front(general_prefabs::new_intro_to_play());

    return ent_vec;

}

pub fn make_deck_list() -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_to_ready());

    return ent_vec;

}

pub fn make_ready_scene() -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_back(general_prefabs::new_battle_notif());

    ent_vec.push_front(general_prefabs::new_screen_manager());

    ent_vec.push_front(general_prefabs::new_swipe_right());
    ent_vec.push_front(general_prefabs::new_swipe_left());

    ent_vec.push_front(general_prefabs::new_reset());

    // Adding profile screen to the right

    let mut player_info_screen = VecDeque::new();

    player_info_screen.push_front(general_prefabs::new_swipe_left());

    apply_screen_offset(&mut player_info_screen, screen().w() as i32);

    ent_vec.append(&mut player_info_screen);

    // Adding profile screen to the right

    let mut player_mult_screen = VecDeque::new();

    player_mult_screen.push_front(general_prefabs::new_swipe_right());
    player_mult_screen.push_front(general_prefabs::new_interaction_info());

    apply_screen_offset(&mut player_mult_screen, screen().w() as i32 * -1);

    ent_vec.append(&mut player_mult_screen);

    return ent_vec;

}

pub fn make_battle_scene() -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_front(general_prefabs::new_screen_manager());
    ent_vec.push_back(general_prefabs::new_swipe_left());

    // Adding profile screen to the right

    let mut player_mult_screen = VecDeque::new();

    player_mult_screen.push_front(general_prefabs::new_swipe_right());
    player_mult_screen.push_front(general_prefabs::new_interaction_info());

    apply_screen_offset(&mut player_mult_screen, screen().w() as i32 * -1);

    ent_vec.append(&mut player_mult_screen);

    return ent_vec;

}

pub fn make_shop_scene() -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();
    
    ent_vec.push_front(general_prefabs::new_screen_manager());
    ent_vec.push_back(general_prefabs::new_swipe_left());

    ent_vec.push_front(general_prefabs::new_reset());

    // Adding profile screen to the right

    let mut player_mult_screen = VecDeque::new();

    player_mult_screen.push_front(general_prefabs::new_swipe_right());
    player_mult_screen.push_front(general_prefabs::new_interaction_info());

    apply_screen_offset(&mut player_mult_screen, screen().w() as i32 * -1);

    ent_vec.append(&mut player_mult_screen);

    return ent_vec;

}

pub fn make_live_feed_scene () -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();

    ent_vec.push_front(general_prefabs::new_reset());

    return ent_vec;

}

pub fn make_misc_scene() -> VecDeque<Entity> {

    let mut ent_vec = VecDeque::new();


    return ent_vec;

}
use crate::{turbe, GameState};

use turbe::{entity::Entity};

use turbe::helpers;
use helpers::{transform::Transform};

use turbe::component_system::component::Component;

// Main Lifecycle System
pub trait ComponentLifecycle {
    fn on_init(&mut self);

    fn on_awake(&mut self);

    fn on_start(&mut self);

    fn on_update(&mut self, ent : &mut Entity<Component>, _state : &mut GameState);

    fn on_destroy(&mut self);

    fn render(&self, transform : Transform,  _state : &mut GameState);
}
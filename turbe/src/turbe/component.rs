use turbo::prelude::*;

use crate::turbe::components::comp_rect;

use super::entity::Entity;
use super::transform::Transform;
use super::size::Size;
use super::border::Border;

pub trait ComponentLifecycle {
    fn on_init(&mut self);

    fn on_awake(&mut self);

    fn on_start(&mut self);

    fn on_update(&mut self, ent : &mut Entity<Component>);

    fn on_destroy(&mut self);

    fn render(&mut self, _x: i32, _y: i32);
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Component {
    Rectangle { transform: Transform, size: Size, color: u32, border: Border},
    Text { text: String },
}

impl Component {

}

impl ComponentLifecycle for Component {
    fn on_init(&mut self) {
        // todo!();
    }

    fn on_awake(&mut self) {
        // todo!();
    }

    fn on_start(&mut self) {
        // todo!();
    }

    fn on_update(&mut self, ent : &mut Entity<Component>) {
        // todo!();
    }

    fn on_destroy(&mut self) {
        // todo!();
    }

    fn render(&mut self, _x: i32, _y: i32) {
        match self {
            Self::Rectangle {transform, size, color, border } => {
                comp_rect::render_rect(*transform, *size, *color, *border, _x, _y);
            },
            Self::Text {text} => {
                text!(text);
            }
        }
    }
}
use turbo::prelude::*;

use super::entity::Entity;

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
    Rectangle { width: u32 },
    Text { text: String },
}

impl Component {}

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
            Self::Rectangle { width } => {
                rect!(
                    x = 112,
                    y = 56,
                    w = *width,
                    h = 32,
                    color = 0x0000ffff,
                    rotation = 45,
                    border_size = 2,
                    border_color = 0xffffffff,
                    border_radius = 4,
                );
            },
            Self::Text {text} => {
                text!(text);
            }
        }
    }
}
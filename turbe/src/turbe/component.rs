use turbo::prelude::*;

use super::entity::Entity;
use super::transform::Transform;
use super::size::Size;

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
    Rectangle { transform: Transform, size: Size },
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
            Self::Rectangle { size, transform } => {
                rect!(
                    x = transform.get_x(),
                    y = transform.get_y(),
                    w = size.get_width() as f32 * size.get_scale_x(),
                    h = size.get_height(),
                    color = 0x0000ffff,
                    rotation = transform.get_rotation(),
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
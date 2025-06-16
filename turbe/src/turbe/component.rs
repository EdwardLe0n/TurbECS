use std::default;

use turbo::prelude::*;

use crate::turbe::components::{comp_rect, comp_spr, comp_text};
use comp_rect::RectangleComponent;
use comp_spr::SpriteComponent;
use comp_text::TextComponent;

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
    Rectangle ( RectangleComponent ),
    Text ( TextComponent ),
    Sprite ( SpriteComponent )
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
        if let Self::Text( text) = self {
            
        }
    }

    fn on_update(&mut self, ent : &mut Entity<Component>) {
        // todo!();
    }

    fn on_destroy(&mut self) {
        // todo!();
    }

    fn render(&mut self, _x: i32, _y: i32) {
        match self {
            Self::Rectangle (rectangle_component ) => {
                comp_rect::render_rect(rectangle_component.clone());
            },
            Self::Text ( text_component) => {
                text!(&text_component.text);
            },
            Self::Sprite ( sprite_component ) => {
                comp_spr::render_sprite(sprite_component.clone());
            },
            default => {}
        }
    }
}

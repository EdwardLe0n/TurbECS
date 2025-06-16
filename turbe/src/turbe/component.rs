use std::default;

use turbo::prelude::*;

use crate::turbe::components::comp_move::MoveComponent;
use crate::turbe::components::{comp_rect, comp_spr, comp_text};
use crate::turbe::transform::{self, Transform};
use comp_rect::RectangleComponent;
use comp_spr::SpriteComponent;
use comp_text::TextComponent;

use super::entity::Entity;
use super::position::Position;
use super::size::Size;
use super::border::Border;

pub trait ComponentLifecycle {
    fn on_init(&mut self);

    fn on_awake(&mut self);

    fn on_start(&mut self);

    fn on_update(&mut self, ent : &mut Entity<Component>);

    fn on_destroy(&mut self);

    fn render(&mut self, transform : Transform);
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum Component {
    Rectangle ( RectangleComponent ),
    Text ( TextComponent ),
    Sprite ( SpriteComponent ),
    Move ( MoveComponent )
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
        match self {
            Self::Move( move_component ) => {
                move_component.update(&mut ent.transform);
            },
            default => {}            
        }
    }

    fn on_destroy(&mut self) {
        // todo!();
    }

    fn render(&mut self, transform : Transform) {
        match self {
            Self::Rectangle (rectangle_component ) => {
                rectangle_component.render_rect(transform);
            },
            Self::Text ( text_component) => {
                text_component.render();
            },
            Self::Sprite ( sprite_component ) => {
                sprite_component.render_sprite(transform);
            },
            default => {}
        }
    }
}

use std::default;

use turbo::prelude::*;

use crate::turbe;

use turbe::{entity::Entity};

use turbe::helpers;
use helpers::{position::Position, size::Size, border::Border, transform::Transform};

use turbe::components;
use components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent, comp_text::TextComponent};

// User made components
use crate::assets::components::comp_move::MoveComponent;

// Main Lifecycle System
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

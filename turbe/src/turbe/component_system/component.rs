use turbo::prelude::*;

use crate::{turbe, GameState};

use turbe::{entity::Entity};

use turbe::helpers;
use helpers::{transform::Transform};

use turbe::component_system;
use component_system::{components, component_lifecycle::ComponentLifecycle};
use components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent, comp_text::TextComponent, comp_butn::ButtonComponent};

// User made components
use crate::assets::components::{comp_move::MoveComponent, comp_increment::IncrementComponent};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct Component {
    pub active : bool,
    pub component_data : ComponentData
}

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub enum ComponentData {
    Rectangle ( RectangleComponent ),
    Text ( TextComponent ),
    Sprite ( SpriteComponent ),
    Button ( ButtonComponent ),

    // User made components

    Move ( MoveComponent ),
    Increment( IncrementComponent )
}

impl Component {

    pub fn new(_component_data : ComponentData) -> Component{
        return Component { active: true, component_data: _component_data }
    }

}

impl ComponentLifecycle for Component {
    fn on_init(&mut self) {
        // todo!();
    }

    fn on_awake(&mut self) {
        // todo!();
    }

    fn on_start(&mut self) {
        
    }

    fn on_update(&mut self, _ent : &mut Entity<Component>, _state : &mut GameState) {
        match &mut self.component_data {
            
            // Standard components

            ComponentData::Button( button_component ) => {
                button_component.update(_ent, _state);
            },

            // User made components

            ComponentData::Move( move_component ) => {
                move_component.update(&mut _ent.transform);
            },
            ComponentData::Increment( increment_component ) => {
                increment_component.update(_state);
            },
            _default => {}            
        }
    }

    fn on_destroy(&mut self) {
        // todo!();
    }

    fn render(&self, _transform : Transform, _state : &mut GameState) {
        match &self.component_data {
            ComponentData::Rectangle (rectangle_component ) => {
                rectangle_component.render_rect(_transform);
            },
            ComponentData::Text ( text_component) => {
                text_component.render(_transform);
            },
            ComponentData::Sprite ( sprite_component ) => {
                sprite_component.render_sprite(_transform);
            },
            ComponentData::Button( button_component ) => {
                button_component.render(_transform);
            },

            // User made components

            ComponentData::Increment(increment_component) => {
                increment_component.render_increment(_transform);
            },

            // Space for edge case

            _default => {}
        }
    }
}

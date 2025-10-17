use turbo::*;

use crate::{turbecs, GameState};

use turbecs::{entity::Entity};

use turbecs::helpers;
use helpers::{transform::Transform};
use helpers::has_x::HasX;

use turbecs::component_system;
use component_system::{components, component_types::ComponentTypes};
use components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent, comp_text::TextComponent};
use components::{comp_text_box::TextBoxComponent, comp_butn::ButtonComponent, comp_particle::ParticleComponent};

// User made components
use crate::assets;

use assets::components::{misc_components};

use misc_components::{comp_resizer::ResizerComponent, comp_textbox_resizer::TextBoxResizerComponent};
use misc_components::{comp_text_box_filler::TextBoxFillerComponent};
use misc_components::{comp_fade::FadeComponent};

#[turbo::serialize]
#[derive(PartialEq)]
pub struct Component {
    pub active : bool,
    pub has : HasX,
    pub ent_locat : usize,
    pub component_data : ComponentData
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum ComponentData {
    Rectangle ( RectangleComponent ),
    Text ( TextComponent ),
    TextBox ( TextBoxComponent ),
    Sprite ( SpriteComponent ),
    Button ( ButtonComponent ),
    Particle ( ParticleComponent ),

    // User made components

    Resizer (ResizerComponent),
    TextBoxResizer (TextBoxResizerComponent),
    Fade (FadeComponent),
    TextBoxFiller (TextBoxFillerComponent),
}

impl Component {

    pub fn new(_component_data : ComponentData) -> Component{
        return Component { active: true, has : HasX::new(), ent_locat : 0, component_data: _component_data }
    }

    pub fn get_comp_type(&self) -> ComponentTypes{
        match &self.component_data {

            // Main components

            ComponentData::Button(_) => {
                return ComponentTypes::Button;
            },
            ComponentData::Rectangle(_) => {
                return ComponentTypes::Rectangle;
            },
            ComponentData::Sprite(_) => {
                return ComponentTypes::Sprite;
            },
            ComponentData::Text(_) => {
                return ComponentTypes::Text;
            },
            ComponentData::TextBox(_) => {
                return ComponentTypes::TextBox;
            },
            ComponentData::Particle(_) => {
                return ComponentTypes::Particle;
            },

            // User made components

            
            
            // Edge case

            _default => {
                return ComponentTypes::Other;
            }
        }


    }

}

impl Component {

    pub fn on_awake(&mut self, ent : &mut Entity, state : &mut GameState) {

        match &mut self.component_data {

            ComponentData::Button( button_component ) => {
                button_component.on_awake(ent, state);
            },

            // User made components

            ComponentData::Resizer( resizer_component ) => {
                resizer_component.on_awake(ent, state);
            },

            ComponentData::TextBoxResizer( textbox_resizer_component ) => {
                textbox_resizer_component.on_awake(ent, state);
            },

            ComponentData::TextBoxFiller(tb_filler_component) => {
                tb_filler_component.on_awake(ent, state);
            },

            // Space for edge case

            _default => {}
        }

    }

    pub fn on_start(&mut self, ent : &mut Entity, state : &mut GameState) {

        match &mut self.component_data {

            ComponentData::Button( button_component ) => {
                button_component.on_start(ent, state);
            },

            // User made components

            

            // Space for edge case

            _default => {}
        }
        
    }

    pub fn on_update(&mut self, ent : &mut Entity, state : &mut GameState) {
        match &mut self.component_data {
            
            // Standard components

            ComponentData::Button( button_component ) => {
                button_component.update(ent, state);
            },

            // User made components

            ComponentData::TextBoxFiller(tb_filler_component) => {
                tb_filler_component.update(ent, state);
            },

            ComponentData::Fade(fade_component) => {
                fade_component.update(ent, state);
            },
            
            _default => {}            
        }
    }

    pub fn on_destroy(&mut self, state : &mut GameState) {
        // todo!();
    }

    pub fn on_render(&self, _transform : Transform, state : &mut GameState) {
        match &self.component_data {
            ComponentData::Button( button_component ) => {
                button_component.render(_transform, state);
            },
            ComponentData::Rectangle (rectangle_component ) => {
                rectangle_component.render_rect(_transform);
            },
            ComponentData::Sprite ( sprite_component ) => {
                sprite_component.render_sprite(_transform);
            },
            ComponentData::Text ( text_component) => {
                text_component.render(_transform);
            },
            ComponentData::TextBox ( text_box_component) => {
                text_box_component.render(_transform);
            },

            // User made components

            

            // Space for edge case

            _default => {}
        }
    }
}

use turbo::prelude::*;

// Core directories

use crate::turbe;
use turbe::helpers;

use crate::GameState;

// Necessary imports

use turbe::entity::Entity;
use turbe::component_system;

use component_system::component::Component;

use component_system::components::buttons::button_types::ButtonTypes;
use helpers::{transform::Transform, position::Position, size::Size, border::Border};

#[derive(Debug, Clone, PartialEq, BorshSerialize, BorshDeserialize)]
pub struct ButtonComponent {
    pub transform : Transform,
    pub button_type : ButtonTypes
}

impl ButtonComponent {

    pub fn new() -> ButtonComponent {
        return ButtonComponent{
            transform : Transform::new(), button_type : ButtonTypes::Default};
    }

}

impl ButtonComponent {

    pub fn update(&mut self, ent : &mut Entity<Component>, _state : &mut GameState) {

        let canvas_bounds = bounds::canvas();

        let bounds = Bounds::with_size(self.transform.get_width() as f32 * self.transform.get_scale_x(), 
                                            self.transform.get_height() as f32 * self.transform.get_scale_y())
                                    .anchor_center(&canvas_bounds);

    }

    pub fn render(&self, _transform : Transform) {

        let canvas_bounds = bounds::canvas();

        let bounds = Bounds::with_size(self.transform.get_width() as f32 * self.transform.get_scale_x() * _transform.get_scale_x(), 
                                            self.transform.get_height() as f32 * self.transform.get_scale_y() * _transform.get_scale_y())
                                    .anchor_center(&canvas_bounds)
                                    .translate(self.transform.get_x() + _transform.get_x(),  -self.transform.get_y() + -_transform.get_y());

        rect!(
            color = 0xffffffff,
            xy = bounds.xy(),
            wh = bounds.wh()
        );

    }

}

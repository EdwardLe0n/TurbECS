use crate::{turbecs, GameState};

use turbecs::managers::particlemanager::{BurstConfig, BurstSource};
use super::particles::{particle_types::ParticleType};

use turbecs::helpers::{position::Position, transform::Transform};

// Core directories

#[turbo::serialize]
#[derive(PartialEq)]
pub struct ParticleComponent {
    pub offset : Position,
    pub config : BurstConfig,
    pub prtcle_type : ParticleType
}

impl ParticleComponent {

    pub fn new(some_config : BurstConfig, some_type : ParticleType) -> Self {

        return Self{offset : Position::new(), config : some_config, prtcle_type : some_type};

    }

    pub fn new_w_offset(some_position : Position, some_config : BurstConfig, some_type : ParticleType) -> Self {

        let mut temp = Self::new(some_config, some_type);

        temp.offset = some_position;

        return temp;

    }

}

impl ParticleComponent {

    pub fn generate(&mut self, transform : Transform, state : &mut GameState) {

        // First does any transform adjustments

        if let BurstSource::Point(_x, _y) = &mut self.config.source {
            
            self.config.source = BurstSource::Point(
                (self.offset.get_x_offset(0, 0) + transform.get_x_offset()) as f32,
                (self.offset.get_y_offset(0, 0) + transform.get_y_offset()) as f32
            );

        }

        if let BurstSource::Circle{ center: _, radius } = &mut self.config.source {
            
            self.config.source = BurstSource::Circle{
                center : (
                    (self.offset.get_x_offset(0, 0) + transform.get_x_offset()) as f32,
                    (self.offset.get_y_offset(0, 0) + transform.get_y_offset()) as f32
                ), 
                radius : *radius};

        }

        // Then will handle any generation

        match &self.prtcle_type.clone() {
            ParticleType::Explode(amount, _) => {

                for _ in 0..amount.clone() {
                    self.generate_particle(state);
                }

            },
            _default => {}
        }

    }

    pub fn generate_particle(&mut self, state : &mut GameState) {

        match &self.prtcle_type.clone() {
            ParticleType::Explode(_, speed) => {

                let mut config = self.config.clone();

                // speed adjustments

                let speed_to_use = speed.clone() as f32;

                config.x_velocity.0 *= speed_to_use;
                config.x_velocity.1 *= speed_to_use;

                config.y_velocity.0 *= speed_to_use;
                config.y_velocity.1 *= speed_to_use;

                state.particle_manager.create_burst(&config);

            },
            _default => {}
        }
        
    }

}
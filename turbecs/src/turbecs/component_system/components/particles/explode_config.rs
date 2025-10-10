use crate::turbecs;

use turbecs::managers::particlemanager::{BurstConfig, BurstSource, Shape};

pub fn explosion() -> BurstConfig {
    BurstConfig {
        source: BurstSource::Circle {
            center: (0.0, 0.0),
            radius: 1.0,
        },
        x_velocity: (-1.0, 1.0),
        y_velocity: (-1.0, 1.0),
        lifetime: (0.4, 1.0),
        color: 0xffffff88,
        size: (4, 6),
        count: 10,
        shape: Shape::Circle,
        should_fade_out: false,
    }
}
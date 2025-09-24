use turbo::*;

#[turbo::serialize]
#[derive(PartialEq)]
pub enum BurstSource {
    Point(f32, f32),
    Circle { center: (f32, f32), radius: f32 },
    Rectangle { min: (f32, f32), max: (f32, f32) },
}

#[turbo::serialize]
#[derive(PartialEq)]
pub enum Shape {
    Square,
    Circle,
    Sprite { name: String },
}

#[turbo::serialize]
pub struct Particle {
    pub pos: (f32, f32),
    pub vel: (f32, f32),
    pub size: u32,
    pub color: u32,
    pub lifetime: f32,
    pub remaining_life: f32,
    pub shape: Shape,
    pub should_fade_out: bool,
}

#[turbo::serialize]
#[derive(PartialEq)]
pub struct BurstConfig {
    pub source: BurstSource,
    pub shape: Shape,
    pub x_velocity: (f32, f32), // (min, max)
    pub y_velocity: (f32, f32),
    pub lifetime: (f32, f32),
    pub color: u32,
    pub size: (u32, u32),
    pub count: u32,
    pub should_fade_out: bool,
}

impl BurstConfig {}

#[turbo::serialize]
pub struct ParticleManager {
    pub bursts: Vec<Vec<Particle>>,
}

impl ParticleManager {
    pub fn new() -> Self {
        Self { bursts: Vec::new() }
    }

    pub fn create_burst(&mut self, config: &BurstConfig) {
        let mut burst = Vec::new();
        for _ in 0..config.count {
            burst.push(self.create_particle(config));
        }
        self.bursts.push(burst);
    }

    // Helper function for generating random float values in a range
    fn rand_float(&self, min: f32, max: f32) -> f32 {
        let range = (max - min).abs();
        if range < 0.001 {
            return min;
        }

        // Handle both small and large ranges consistently
        let scaled_range = (range * 1000.0) as u32;
        if scaled_range == 0 {
            return min;
        }

        min + (random::u32() % scaled_range) as f32 / 1000.0
    }

    // Helper function for generating random integer values in a range
    fn rand_int(&self, min: u32, max: u32) -> u32 {
        if max <= min {
            return min;
        }
        min + (random::u32() % (max - min))
    }

    fn create_particle(&self, config: &BurstConfig) -> Particle {
        // Get position based on source type
        let pos = match &config.source {
            BurstSource::Point(x, y) => (*x, *y),
            BurstSource::Circle { center, radius } => {
                let angle = self.rand_float(0.0, std::f32::consts::TAU);
                let dist = self.rand_float(0.0, *radius);
                (center.0 + dist * angle.cos(), center.1 + dist * angle.sin())
            }
            BurstSource::Rectangle { min, max } => {
                let x = self.rand_float(min.0, max.0);
                let y = self.rand_float(min.1, max.1);
                (x, y)
            }
        };

        // Get random velocities, lifetime, and size using helper functions
        let vx = self.rand_float(config.x_velocity.0, config.x_velocity.1);
        let vy = self.rand_float(config.y_velocity.0, config.y_velocity.1);
        let lifetime = self.rand_float(config.lifetime.0, config.lifetime.1);
        let size = self.rand_int(config.size.0, config.size.1);
        let shape = config.shape.clone();

        Particle {
            pos,
            vel: (vx, vy),
            color: config.color,
            lifetime,
            remaining_life: lifetime,
            size,
            shape,
            should_fade_out: config.should_fade_out,
        }
    }

    pub fn update(&mut self) {
        // Update remaining life and remove dead particles from each burst
        for burst in &mut self.bursts {
            for particle in burst.iter_mut() {
                particle.pos.0 += particle.vel.0;
                particle.pos.1 += particle.vel.1;
                particle.remaining_life -= 1.0 / 60.0;
                if particle.should_fade_out {
                    let life_progress = 1.0 - (particle.remaining_life / particle.lifetime);

                    if life_progress > 0.5 {
                        // Map 0.5-1.0 to 255-0 for alpha
                        let alpha = ((1.0 - life_progress) * 2.0 * 255.0) as u32;
                        particle.color = (particle.color & 0xFFFFFF00) | alpha;
                    }
                }
            }
            // Remove dead particles from this burst
            burst.retain(|particle| particle.remaining_life > 0.0);
        }
        // Remove empty bursts
        self.bursts.retain(|burst| !burst.is_empty());
    }

    pub fn draw(&self) {
        for burst in &self.bursts {
            for particle in burst {
                match &particle.shape {
                    Shape::Square => {
                        rect!(
                            x = particle.pos.0,
                            y = particle.pos.1,
                            w = particle.size,
                            h = particle.size,
                            color = particle.color
                        );
                    }
                    Shape::Circle => {
                        circ!(
                            x = particle.pos.0,
                            y = particle.pos.1,
                            d = particle.size,
                            color = particle.color
                        );
                    }
                    Shape::Sprite { name } => {
                        sprite!(&name, x = particle.pos.0, y = particle.pos.1);
                    }
                }
            }
        }
    }
}

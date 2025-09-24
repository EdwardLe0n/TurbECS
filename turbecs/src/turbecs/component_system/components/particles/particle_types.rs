#[turbo::serialize]
#[derive(PartialEq)]
pub enum ParticleType {

    Explode(u32, u32),
    Stream,
    Default

}

impl ParticleType {
    
    pub fn new_explode(amount : u32, speed : u32) -> Self {

        return ParticleType::Explode(amount, speed);

    }

}
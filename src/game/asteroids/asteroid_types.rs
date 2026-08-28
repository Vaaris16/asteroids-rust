use rand::RngExt;

pub enum AsteroidType {
    AsteroidSmall,
    AsteroidMedium,
    AsteroidLarge,
}

impl AsteroidType {
    pub fn path(&self) -> &'static str {
        match self {
            AsteroidType::AsteroidSmall => "asteroids_images/asteroid_small.png",
            AsteroidType::AsteroidMedium => "asteroids_images/asteroid_medium.png",
            AsteroidType::AsteroidLarge => "asteroids_images/asteroid_large.png",
        }
    }

    pub fn rand_asteroid_type() -> AsteroidType {
        match rand::rng().random_range(0..3) {
            0 => AsteroidType::AsteroidSmall,
            1 => AsteroidType::AsteroidMedium,
            2 => AsteroidType::AsteroidLarge,
            _ => unreachable!(),
        }
    }
}

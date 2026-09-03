use rand::RngExt;

// Enum representing the different types of asteroids.
#[derive(Clone)]
pub enum AsteroidType {
    AsteroidSmall,
    AsteroidMedium,
    AsteroidLarge,
}

impl AsteroidType {
    // Returns a random AsteroidType.
    pub fn rand_asteroid_type() -> AsteroidType {
        match rand::rng().random_range(0..3) {
            0 => AsteroidType::AsteroidSmall,
            1 => AsteroidType::AsteroidMedium,
            2 => AsteroidType::AsteroidLarge,
            _ => unreachable!(),
        }
    }

    // Returns the next asteroid type.
    pub fn next_asteroid_type(&self) -> Option<AsteroidType> {
        match self {
            AsteroidType::AsteroidLarge => Some(AsteroidType::AsteroidMedium),
            AsteroidType::AsteroidMedium => Some(AsteroidType::AsteroidSmall),
            AsteroidType::AsteroidSmall => None,
        }
    }
}

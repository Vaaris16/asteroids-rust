use rand::RngExt;

// Enum representing the different types of asteroids.
#[derive(Clone)]
pub enum AsteroidType {
    Small,
    Medium,
    Large,
}

impl AsteroidType {
    // Returns a random AsteroidType.
    pub fn rand_asteroid_type() -> AsteroidType {
        match rand::rng().random_range(0..3) {
            0 => AsteroidType::Small,
            1 => AsteroidType::Medium,
            2 => AsteroidType::Large,
            _ => unreachable!(),
        }
    }

    // Returns the next asteroid type.
    pub fn next_asteroid_type(&self) -> Option<AsteroidType> {
        match self {
            AsteroidType::Large => Some(AsteroidType::Medium),
            AsteroidType::Medium => Some(AsteroidType::Small),
            AsteroidType::Small => None,
        }
    }
}

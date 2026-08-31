use rand::RngExt;

// Enum representing the different types of asteroids.
pub enum AsteroidType {
    AsteroidSmall,
    AsteroidMedium,
    AsteroidLarge,
}

const ASTEROID_SMALL_IMAGE_PATH: &str = "asteroids_images/asteroid_small.png";
const ASTEROID_MEDIUM_IMAGE_PATH: &str = "asteroids_images/asteroid_medium.png";
const ASTEROID_LARGE_IMAGE_PATH: &str = "asteroids_images/asteroid_large.png";

impl AsteroidType {
    // Returns the image path based on the AsteroidType.
    pub fn path(&self) -> &'static str {
        match self {
            AsteroidType::AsteroidSmall => ASTEROID_SMALL_IMAGE_PATH,
            AsteroidType::AsteroidMedium => ASTEROID_MEDIUM_IMAGE_PATH,
            AsteroidType::AsteroidLarge => ASTEROID_LARGE_IMAGE_PATH,
        }
    }

    // Returns a random AsteroidType.
    pub fn rand_asteroid_type() -> AsteroidType {
        match rand::rng().random_range(0..3) {
            0 => AsteroidType::AsteroidSmall,
            1 => AsteroidType::AsteroidMedium,
            2 => AsteroidType::AsteroidLarge,
            _ => unreachable!(),
        }
    }
}

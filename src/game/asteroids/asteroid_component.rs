use crate::game::asteroids::{asteroid_sides::Side, asteroid_types::AsteroidType};
use bevy::prelude::*;
use rand::RngExt;

const SMALL_ASTEROID_RADIUS: f32 = 25.;
const MEDIUM_ASTEROID_RADIUS: f32 = 50.;
const LARGE_ASTEROID_RADIUS: f32 = 100.;

const DEFAULT_VELOCITY_ASTEROID: Vec3 = Vec3::ZERO;

const MIN_ROTATION_FACTOR_ASTEROID: f32 = 0.01;
const MAX_ROTATION_FACTOR_ASTEROID: f32 = 0.03;

const ASTEROID_PATH_1: &str = "asteroids_images/asteroid_1.png";
const ASTEROID_PATH_2: &str = "asteroids_images/asteroid_2.png";
const ASTEROID_PATH_3: &str = "asteroids_images/asteroid_3.png";

// Component used to identify asteroids
#[derive(Component)]
pub struct Asteroid {
    pub velocity: Vec3,
    pub side: Side,
    pub rotation_factor: f32,
    pub window_x: f32,
    pub window_y: f32,
    pub asteroid_type: AsteroidType,
    pub asteroid_path: &'static str,
    pub collider_radius: f32,
}

impl Asteroid {
    // Used to spawn a new Asteroid Component
    pub fn new(window_x: f32, window_y: f32, asteroid_type: AsteroidType) -> Self {
        let collider_radius = Self::get_collider_radius(&asteroid_type);
        Self {
            velocity: DEFAULT_VELOCITY_ASTEROID,
            side: Side::rand_side(),
            rotation_factor: Self::rand_rotation_factor(),
            window_x,
            window_y,
            asteroid_type,
            asteroid_path: Self::asteroid_path(),
            collider_radius,
        }
    }
    // Returns the asteroid image's path.
    fn asteroid_path() -> &'static str {
        match rand::rng().random_range(0..3) {
            0 => ASTEROID_PATH_1,
            1 => ASTEROID_PATH_2,
            2 => ASTEROID_PATH_3,
            _ => unreachable!(),
        }
    }
    // Returns random velocity.
    pub fn rand_vel() -> Vec3 {
        let mut rng = rand::rng();
        let vel = Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(-5.0..5.0), 0.);
        vel
    }
    // Returns the collider_radius based on the AsteroidType.
    fn get_collider_radius(asteroid_type: &AsteroidType) -> f32 {
        match asteroid_type {
            AsteroidType::AsteroidSmall => SMALL_ASTEROID_RADIUS,
            AsteroidType::AsteroidMedium => MEDIUM_ASTEROID_RADIUS,
            AsteroidType::AsteroidLarge => LARGE_ASTEROID_RADIUS,
        }
    }
    // Returns a random rotation factor
    fn rand_rotation_factor() -> f32 {
        rand::rng().random_range(MIN_ROTATION_FACTOR_ASTEROID..MAX_ROTATION_FACTOR_ASTEROID)
    }
    // Returns random position and velocity for the asteroid.
    pub fn rand_pos_vel(&self) -> (Vec3, Vec3) {
        let mut rng = rand::rng();
        let (pos_y, pos_x) = (
            Vec3::new(
                rng.random_range(-self.window_x / 2.0..self.window_x / 2.0),
                self.window_y / 2.,
                0.,
            ),
            Vec3::new(
                self.window_x / 2.,
                rng.random_range(-self.window_y / 2.0..self.window_y / 2.0),
                0.,
            ),
        );

        match self.side {
            Side::Top => {
                let vel = Vec3::new(
                    rng.random_range(-3.0..3.0),
                    rng.random_range(-5.0..-2.0),
                    0.,
                );
                (pos_y, vel)
            }

            Side::Bottom => {
                let vel = Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(2.0..5.0), 0.);

                (pos_y, vel)
            }
            Side::Right => {
                let vel = Vec3::new(
                    rng.random_range(-5.0..-2.0),
                    rng.random_range(-3.0..3.0),
                    0.,
                );

                (pos_x, vel)
            }

            Side::Left => {
                let vel = Vec3::new(rng.random_range(2.0..5.0), rng.random_range(-3.0..3.0), 0.);

                (pos_x, vel)
            }
        }
    }
}

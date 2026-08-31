use crate::game::asteroids::{asteroid_sides::Side, asteroid_types::AsteroidType};
use bevy::prelude::*;
use rand::RngExt;

const SMALL_ASTEROID_RADIUS: f32 = 50.;
const MEDIUM_ASTEROID_RADIUS: f32 = 100.;
const LARGE_ASTEROID_RADIUS: f32 = 125.;

const DEFAULT_VELOCITY_ASTEROID: Vec3 = Vec3::ZERO;

const MIN_ROTATION_FACTOR_ASTEROID: f32 = 0.01;
const MAX_ROTATION_FACTOR_ASTEROID: f32 = 0.03;

// Component used to identify asteroids
#[derive(Component)]
pub struct Asteroid {
    pub velocity: Vec3,
    pub side: Side,
    pub rotation_factor: f32,
    pub window_x: f32,
    pub window_y: f32,
    pub asteroid_type: AsteroidType,
    pub collider_radius: f32,
}

impl Asteroid {
    // Used to spawn a new Asteroid Component
    pub fn new(window_x: f32, window_y: f32) -> Self {
        let asteroid_type = AsteroidType::rand_asteroid_type();
        let collider_radius = Self::get_collider_radius(&asteroid_type);
        Self {
            velocity: DEFAULT_VELOCITY_ASTEROID,
            side: Side::rand_side(),
            rotation_factor: Self::rand_rotation_factor(),
            window_x,
            window_y,
            asteroid_type,
            collider_radius,
        }
    }
    // Returns the collider_radius based on the AsteroidType
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
                let vel = Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(-3.0..0.0), 0.);
                (pos_y, vel)
            }

            Side::Bottom => {
                let vel = Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(3.0..6.0), 0.);

                (pos_y, vel)
            }
            Side::Right => {
                let vel = Vec3::new(
                    rng.random_range(-6.0..-3.0),
                    rng.random_range(-3.0..3.0),
                    0.,
                );

                (pos_x, vel)
            }

            Side::Left => {
                let vel = Vec3::new(rng.random_range(3.0..6.0), rng.random_range(-6.0..3.0), 0.);

                (pos_x, vel)
            }
        }
    }
}

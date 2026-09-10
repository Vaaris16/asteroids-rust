use crate::{
    core::game_assets::game_assets::GameAssets,
    game::asteroids::{asteroid_sides::Side, asteroid_types::AsteroidType},
};
use bevy::prelude::*;
use rand::{RngExt, rngs::ThreadRng};

// Defines the radius of the asteroid collider.
const SMALL_ASTEROID_RADIUS: f32 = 20.;
const MEDIUM_ASTEROID_RADIUS: f32 = 40.;
const LARGE_ASTEROID_RADIUS: f32 = 80.;

// Default asteroid velocity
const DEFAULT_VELOCITY_ASTEROID: Vec3 = Vec3::ZERO;

// Defines the min/max rotation factor for the asteroids
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
    pub asteroid_path: Handle<Image>,
    pub collider_radius: f32,
}

impl Asteroid {
    // Used to spawn a new Asteroid Component
    pub fn new(
        window_x: f32,
        window_y: f32,
        asteroid_type: AsteroidType,
        game_assets: &GameAssets,
    ) -> Self {
        let collider_radius = Self::get_collider_radius(&asteroid_type);
        Self {
            velocity: DEFAULT_VELOCITY_ASTEROID,
            side: Side::rand_side(),
            rotation_factor: Self::rand_rotation_factor(),
            window_x,
            window_y,
            asteroid_type,
            asteroid_path: game_assets.rand_asteroids(),
            collider_radius,
        }
    }

    // Returns random velocity.
    pub fn rand_vel() -> Vec3 {
        let mut rng = rand::rng();
        Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(-5.0..5.0), 0.)
    }

    // Returns the collider_radius based on the AsteroidType.
    fn get_collider_radius(asteroid_type: &AsteroidType) -> f32 {
        match asteroid_type {
            AsteroidType::Small => SMALL_ASTEROID_RADIUS,
            AsteroidType::Medium => MEDIUM_ASTEROID_RADIUS,
            AsteroidType::Large => LARGE_ASTEROID_RADIUS,
        }
    }

    // Returns a random rotation factor
    fn rand_rotation_factor() -> f32 {
        rand::rng().random_range(MIN_ROTATION_FACTOR_ASTEROID..MAX_ROTATION_FACTOR_ASTEROID)
    }

    // Returns a random velocity based on the Side of the asteroid.
    fn rand_vel_based_sides(&self, rng: &mut ThreadRng) -> Vec3 {
        match self.side {
            Side::Top => Vec3::new(
                rng.random_range(-3.0..3.0),
                rng.random_range(-5.0..-2.0),
                0.,
            ),

            Side::Bottom => Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(2.0..5.0), 0.),
            Side::Right => Vec3::new(
                rng.random_range(-5.0..-2.0),
                rng.random_range(-3.0..3.0),
                0.,
            ),

            Side::Left => Vec3::new(rng.random_range(2.0..5.0), rng.random_range(-3.0..3.0), 0.),
        }
    }

    // Returns a random position.
    fn rand_pos(&self, rng: &mut ThreadRng) -> Vec3 {
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
            Side::Top => pos_y,
            Side::Bottom => pos_y,
            Side::Right => pos_x,
            Side::Left => pos_x,
        }
    }

    // Returns random position and velocity for the asteroid.
    pub fn rand_pos_vel(&self) -> (Vec3, Vec3) {
        let mut rng = rand::rng();
        (self.rand_pos(&mut rng), self.rand_vel_based_sides(&mut rng))
    }
}

use crate::game::asteroids::{asteroid_plugin::Side, asteroid_types::AsteroidType};
use bevy::prelude::*;
use rand::RngExt;

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
    pub fn new(window_x: f32, window_y: f32) -> Self {
        let asteroid_type = AsteroidType::rand_asteroid_type();
        let collider_radius = Self::get_collider_radius(&asteroid_type);
        Self {
            velocity: Vec3::ZERO,
            side: Self::rand_side(),
            rotation_factor: Self::rand_rotation_factor(),
            window_x,
            window_y,
            asteroid_type,
            collider_radius,
        }
    }
    fn get_collider_radius(asteroid_type: &AsteroidType) -> f32 {
        match asteroid_type {
            AsteroidType::AsteroidSmall => 50.,
            AsteroidType::AsteroidMedium => 100.,
            AsteroidType::AsteroidLarge => 125.,
        }
    }
    fn rand_rotation_factor() -> f32 {
        rand::rng().random_range(0.01..0.03)
    }
    fn rand_side() -> Side {
        let mut rng = rand::rng();
        match rng.random_range(0..4) {
            0 => Side::Top,
            1 => Side::Bottom,
            2 => Side::Right,
            3 => Side::Left,
            _ => unreachable!(),
        }
    }
    pub fn rand_pos_vel(&self) -> (Vec3, Vec3) {
        match self.side {
            Side::Top => {
                let mut rng = rand::rng();
                let pos = Vec3::new(
                    rng.random_range(-self.window_x / 2.0..self.window_x / 2.0),
                    self.window_y / 2.,
                    0.,
                );

                let vel = Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(-3.0..0.0), 0.);

                (pos, vel)
            }

            Side::Bottom => {
                let mut rng = rand::rng();
                let pos = Vec3::new(
                    rng.random_range(-self.window_x / 2.0..self.window_x / 2.0),
                    -self.window_y / 2.,
                    0.,
                );

                let vel = Vec3::new(rng.random_range(-3.0..3.0), rng.random_range(3.0..6.0), 0.);

                (pos, vel)
            }
            Side::Right => {
                let mut rng = rand::rng();
                let pos = Vec3::new(
                    self.window_x / 2.,
                    rng.random_range(-self.window_y / 2.0..self.window_y / 2.0),
                    0.,
                );

                let vel = Vec3::new(
                    rng.random_range(-6.0..-3.0),
                    rng.random_range(-3.0..3.0),
                    0.,
                );

                (pos, vel)
            }

            Side::Left => {
                let mut rng = rand::rng();
                let pos = Vec3::new(
                    -self.window_x / 2.,
                    rng.random_range(-self.window_y / 2.0..self.window_y / 2.0),
                    0.,
                );

                let vel = Vec3::new(rng.random_range(3.0..6.0), rng.random_range(-6.0..3.0), 0.);

                (pos, vel)
            }
        }
    }
}

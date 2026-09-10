use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};

use crate::{
    GameState,
    core::game_assets::game_assets::GameAssets,
    game::{
        asteroids::{asteroid_component::Asteroid, asteroid_types::AsteroidType},
        game_plugin::GameSet,
    },
};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                maintain_asteroids,
                out_of_bounds_asteroid,
                rotate_asteroids,
                move_asteroid,
            )
                .in_set(GameSet),
        )
        .add_systems(OnExit(GameState::Game), cleanup_asteroids);
    }
}

// Spawns an asteroid.
pub fn spawn_asteroid(commands: &mut Commands, pos: Vec3, vel: Vec3, mut asteroid: Asteroid) {
    asteroid.velocity = vel;

    commands.spawn((
        Sprite {
            image: asteroid.asteroid_path.clone(),
            custom_size: Some(Vec2::splat(asteroid.collider_radius * 2.)),
            ..Default::default()
        },
        Collider::circle(asteroid.collider_radius),
        CollisionEventsEnabled,
        asteroid,
        Transform::from_translation(pos),
    ));
}

// Moves the asteroids using their velocity.
fn move_asteroid(asteroids: Query<(&mut Transform, &Asteroid), With<Asteroid>>) {
    for (mut asteroid_trans, asteroid) in asteroids {
        asteroid_trans.translation += asteroid.velocity;
    }
}

// Defines the max asteroid.
const MAX_ASTEROIDS: usize = 10;

// Maintains the number of asteroids.
fn maintain_asteroids(
    window_s: Single<&Window>,
    asteroids: Query<(), With<Asteroid>>,
    mut commands: Commands,
    game_assets: Res<GameAssets>,
) {
    let asteroid_count = asteroids.iter().count();

    let asteroid_type = AsteroidType::rand_asteroid_type();
    let asteroid = Asteroid::new(
        window_s.width(),
        window_s.height(),
        asteroid_type,
        &game_assets,
    );
    let (pos, vel) = asteroid.rand_pos_vel();
    if asteroid_count <= MAX_ASTEROIDS {
        spawn_asteroid(&mut commands, pos, vel, asteroid);
    }
}

// Checks whether asteroids are out of bounds and despawns them.
fn out_of_bounds_asteroid(
    asteroids: Query<(&Transform, Entity, &Asteroid), With<Asteroid>>,
    window_s: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    for (asteroid_trans, asteroid_entity, asteroid) in asteroids {
        let asteroid_size = asteroid.collider_radius;
        if asteroid_trans.translation.y > (window_s.height() + asteroid_size)
            || asteroid_trans.translation.y < (-window_s.height() - asteroid_size)
            || asteroid_trans.translation.x > (window_s.width() + asteroid_size)
            || asteroid_trans.translation.x < (-window_s.width() - asteroid_size)
        {
            commands.entity(asteroid_entity).despawn();
        }
    }
}

// Rotates asteroids based on their random rotation_factor.
fn rotate_asteroids(asteroids: Query<(&mut Transform, &Asteroid), With<Asteroid>>) {
    for (mut asteroid_trans, asteroid) in asteroids {
        asteroid_trans.rotate_z(asteroid.rotation_factor);
    }
}

// Removes any remaining asteroids.
fn cleanup_asteroids(mut commands: Commands, asteroids: Query<Entity, With<Asteroid>>) {
    for asteroid in asteroids {
        commands.entity(asteroid).despawn();
    }
}

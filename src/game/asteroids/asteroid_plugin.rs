use avian2d::prelude::*;
use bevy::{log::tracing::instrument, prelude::*, window::PrimaryWindow};
use rand::RngExt;

use crate::{
    GameState,
    game::{
        asteroids::asteroid_component::Asteroid, game_plugin::GameSet,
        spaceship::spaceship_plugin::Bullet,
    },
};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        // Adds avian2d.
        app.add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default());
        app.add_systems(
            Update,
            (
                maintain_asteroids,
                out_of_bounds_asteroid,
                rotate_asteroids,
                move_asteroid,
            )
                .in_set(GameSet),
        );
    }
}

// Enum to define the sides at which the asteroids can spawn.
pub enum Side {
    Top,
    Bottom,
    Right,
    Left,
}

// Spawns an asteroid.
fn spawn_asteroid(
    window_s: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
) {
    let mut asteroid = Asteroid::new(window_s.width(), window_s.height());
    let (pos, vel) = asteroid.rand_pos_vel();
    asteroid.velocity = vel;

    commands.spawn((
        Sprite {
            image: assets_server.load(asteroid.asteroid_type.path()),
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

// Maintains the number of asteroids.
fn maintain_asteroids(
    asteroids: Query<(), With<Asteroid>>,
    window_s: Single<&Window, With<PrimaryWindow>>,
    commands: Commands,
    assets_server: Res<AssetServer>,
) {
    let asteroid_count = asteroids.iter().count();
    if asteroid_count <= 3 {
        spawn_asteroid(window_s, commands, assets_server);
    }
}

// Checks whether asteroids are out of bounds and despawns them.
fn out_of_bounds_asteroid(
    asteroids: Query<(&mut Transform, Entity), With<Asteroid>>,
    window_s: Single<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
) {
    for (asteroid_trans, asteroid_entity) in asteroids {
        if asteroid_trans.translation.y > (window_s.height() + 100.0)
            || asteroid_trans.translation.y < (-window_s.height() - 100.0)
            || asteroid_trans.translation.x > (window_s.width() + 100.0)
            || asteroid_trans.translation.x < (-window_s.height() + -100.0)
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

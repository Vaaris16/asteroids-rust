use avian2d::prelude::*;
use bevy::{prelude::*, window::PrimaryWindow};
use rand::RngExt;

use crate::game::{asteroids::asteroid_component::Asteroid, spaceship::spaceship_plugin::Bullet};

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PhysicsPlugins::default())
            .add_plugins(PhysicsDebugPlugin::default());
        app.add_systems(Update, maintain_asteroids)
            .add_systems(Update, out_of_bounds_asteroid)
            .add_systems(Update, rotate_asteroids)
            .add_systems(Update, move_asteroid)
            .add_systems(Update, check_collision_with_bullet);
    }
}

pub enum Side {
    Top,
    Bottom,
    Right,
    Left,
}

pub enum AsteroidType {
    AsteroidSmall,
    AsteroidMedium,
    AsteroidLarge,
}

impl AsteroidType {
    fn path(&self) -> &'static str {
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

fn move_asteroid(asteroids: Query<(&mut Transform, &Asteroid), With<Asteroid>>) {
    for (mut asteroid_trans, asteroid) in asteroids {
        asteroid_trans.translation += asteroid.velocity;
    }
}

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

fn rotate_asteroids(asteroids: Query<(&mut Transform, &Asteroid), With<Asteroid>>) {
    for (mut asteroid_trans, asteroid) in asteroids {
        asteroid_trans.rotate_z(asteroid.rotation_factor);
    }
}

fn check_collision_with_bullet(
    mut events: MessageReader<CollisionStart>,
    bullets: Query<(), With<Bullet>>,
    asteroids: Query<(), With<Asteroid>>,
    mut commands: Commands,
) {
    for event in events.read() {
        let entity1 = event.collider1;
        let entity2 = event.collider2;

        if bullets.contains(entity1) && asteroids.contains(entity2) {
            commands.entity(entity2).despawn();
        } else if bullets.contains(entity2) && asteroids.contains(entity1) {
            commands.entity(entity1).despawn();
        }
    }
}

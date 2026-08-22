use bevy::{prelude::*, window::PrimaryWindow};
use rand::RngExt;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, maintain_asteroids)
            .add_systems(Update, out_of_bounds_asteroid)
            .add_systems(Update, move_asteroid);
    }
}

enum Side {
    Top,
    Bottom,
    Right,
    Left,
}

#[derive(Component)]
struct Asteroid {
    velocity: Vec3,
    side: Side,
    window_x: f32,
    window_y: f32,
}

impl Asteroid {
    fn new(window_x: f32, window_y: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,
            side: Side::Top,
            window_x,
            window_y,
        }
    }
    fn rand_pos_vel(&self) -> (Vec3, Vec3) {
        match self.side {
            Side::Top => {
                let mut rng = rand::rng();
                let pos = Vec3::new(
                    rng.random_range(-self.window_x / 2.0..self.window_x / 2.0),
                    self.window_y / 2.,
                    0.,
                );

                let vel = Vec3::new(
                    rng.random_range(-3.0..3.0),
                    rng.random_range(-6.0..-3.0),
                    0.,
                );

                (pos, vel)
            }
            _ => (Vec3::splat(0.), Vec3::splat(0.)),
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
            image: assets_server.load("asteroids_images/asteroid_1.png"),
            custom_size: Some(Vec2::splat(200.)),
            ..Default::default()
        },
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
    if asteroid_count <= 4 {
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

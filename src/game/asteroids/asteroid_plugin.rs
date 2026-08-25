use bevy::{prelude::*, window::PrimaryWindow};
use rand::RngExt;

pub struct AsteroidPlugin;

impl Plugin for AsteroidPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, maintain_asteroids)
            .add_systems(Update, out_of_bounds_asteroid)
            .add_systems(Update, rotate_asteroids)
            .add_systems(Update, move_asteroid);
    }
}

enum Side {
    Top,
    Bottom,
    Right,
    Left,
}

enum AsteroidType {
    AsteroidSmall,
    AsteroidMedium,
    AsteroidLarge,
}

impl AsteroidType {
    fn path(&self) -> &'static str {
        match self {
            AsteroidType::AsteroidSmall => "asteroids_images/asteroid_1.png",
            AsteroidType::AsteroidMedium => "asteroids_images/asteroid_2.png",
            AsteroidType::AsteroidLarge => "asteroids_images/asteroid_3.png",
        }
    }

    fn rand_asteroid_type() -> AsteroidType {
        match rand::rng().random_range(0..3) {
            0 => AsteroidType::AsteroidSmall,
            1 => AsteroidType::AsteroidMedium,
            2 => AsteroidType::AsteroidLarge,
            _ => unreachable!(),
        }
    }
}

#[derive(Component)]
pub struct Asteroid {
    velocity: Vec3,
    side: Side,
    rotation_factor: f32,
    window_x: f32,
    window_y: f32,
    asteroid_path: AsteroidType,
}

impl Asteroid {
    fn new(window_x: f32, window_y: f32) -> Self {
        Self {
            velocity: Vec3::ZERO,
            side: Self::rand_side(),
            rotation_factor: Self::rand_rotation_factor(),
            window_x,
            window_y,
            asteroid_path: AsteroidType::rand_asteroid_type(),
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
    fn rand_pos_vel(&self) -> (Vec3, Vec3) {
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
            image: assets_server.load(asteroid.asteroid_path.path()),
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

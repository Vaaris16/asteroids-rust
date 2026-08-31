use avian2d::prelude::*;
use bevy::{log::tracing::Instrument, prelude::*};

use crate::{GameState, game::game_plugin::GameSet};

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn_space_ship)
            .add_systems(Update, space_ship_controls.in_set(GameSet))
            .add_systems(Update, out_of_bounds_bullets.in_set(GameSet))
            .add_systems(OnExit(GameState::Game), cleanup_spaceship);
    }
}

// Image path of the space ship.
const SPACE_SHIP_IMAGE_PATH: &str = "space_ship.png";
// Defines the amount the space ship rotates per update.
const SPACE_SHIP_ROTATION: f32 = 0.1;
// Defines the 3 points used for space ship collider.
const SPACE_SHIP_POINT_A: Vec2 = Vec2::new(-37.5, -50.);
const SPACE_SHIP_POINT_B: Vec2 = Vec2::new(37.5, -50.);
const SPACE_SHIP_POINT_C: Vec2 = Vec2::new(0., -50.);

// Image path of the bullet.
const BULLET_IMAGE_PATH: &str = "bullet.png";
// Bullet offset from the space ship.
const BULLET_OFFSET: Vec3 = Vec3::new(0., 75., 0.);
// Defines the bullet speed.
const BULLET_SPEED: f32 = 600.;
// Defines the width and height of the bullet.
const BULLET_SIZE: [f32; 2] = [15., 45.];

#[derive(Component)]
pub struct SpaceShip;

#[derive(Component)]
pub struct Bullet;

// Spawns the space ship.
fn spawn_space_ship(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: assets_server.load(SPACE_SHIP_IMAGE_PATH),
            ..Default::default()
        },
        Collider::triangle(SPACE_SHIP_POINT_A, SPACE_SHIP_POINT_B, SPACE_SHIP_POINT_C),
        CollisionEventsEnabled,
        RigidBody::Kinematic,
        SpaceShip,
        Transform::default(),
    ));
}

// Defines controls for the space ship.
fn space_ship_controls(
    mut space_ship: Single<&mut Transform, With<SpaceShip>>,
    key_pressed: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
) {
    for key in key_pressed.get_pressed() {
        match key {
            KeyCode::ArrowRight => space_ship.rotate_z(SPACE_SHIP_ROTATION),
            KeyCode::ArrowLeft => space_ship.rotate_z(-SPACE_SHIP_ROTATION),
            _ => (),
        }

        if key_pressed.just_pressed(KeyCode::Space) {
            spawn_bullet(&space_ship, &mut commands, &assets_server);
        }
    }
}

// Spawns a bullet from the tip of the space ship.
fn spawn_bullet(
    space_ship: &Single<&mut Transform, With<SpaceShip>>,
    commands: &mut Commands,
    assets_server: &AssetServer,
) {
    let tip = space_ship.translation + space_ship.rotation * BULLET_OFFSET;
    commands.spawn((
        Sprite {
            image: assets_server.load(BULLET_IMAGE_PATH),
            ..Default::default()
        },
        Bullet,
        Collider::rectangle(BULLET_SIZE[0], BULLET_SIZE[1]),
        CollisionEventsEnabled,
        RigidBody::Kinematic,
        LinearVelocity(((space_ship.rotation * Vec3::Y) * BULLET_SPEED).truncate()),
        Transform {
            translation: tip,
            rotation: space_ship.rotation,
            ..Default::default()
        },
    ));
}

// Removes the space ship
fn cleanup_spaceship(mut commands: Commands, space_ship: Single<Entity, With<SpaceShip>>) {
    commands.entity(*space_ship).despawn();
}

fn out_of_bounds_bullets(
    bullets: Query<(&mut Transform, Entity), With<Bullet>>,
    window_s: Single<&Window>,
    mut commands: Commands,
) {
    let w_width = window_s.width() / 2.;
    let w_height = window_s.height() / 2.;
    for (bullet_trans, bullet_entity) in bullets {
        if bullet_trans.translation.y > w_height
            || bullet_trans.translation.y < -w_height
            || bullet_trans.translation.x > w_width
            || bullet_trans.translation.x < -w_width
        {
            commands.entity(bullet_entity).despawn();
        }
    }
}

use avian2d::prelude::*;
use bevy::prelude::*;

pub struct SpaceShipPlugin;

impl Plugin for SpaceShipPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_space_ship)
            .add_systems(Update, space_ship_controls);
    }
}

const SPACE_SHIP_IMAGE_PATH: &str = "space_ship.png";
const SPACE_SHIP_ROTATION: f32 = 0.1;

const BULLET_IMAGE_PATH: &str = "bullet.png";
const BULLET_OFFSET: Vec3 = Vec3::new(0., 75., 0.);
const BULLET_SPEED: f32 = 600.;

#[derive(Component)]
struct SpaceShip;

#[derive(Component)]
pub struct Bullet {
    velocity: Vec3,
}

fn spawn_space_ship(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        Sprite {
            image: assets_server.load(SPACE_SHIP_IMAGE_PATH),
            ..Default::default()
        },
        SpaceShip,
        Transform::default(),
    ));
}

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
        Bullet {
            velocity: (space_ship.rotation * Vec3::Y) * BULLET_SPEED,
        },
        Collider::rectangle(15., 45.),
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

fn move_bullets(bullets: Query<(&mut Transform, &Bullet), With<Bullet>>) {
    for (mut bullet_trans, bullet) in bullets {
        bullet_trans.translation += bullet.velocity;
    }
}

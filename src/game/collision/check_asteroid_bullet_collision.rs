use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game::{asteroids::asteroid_component::Asteroid, spaceship::spaceship_plugin::Bullet};

pub fn check_collision_asteroid_with_bullet(
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

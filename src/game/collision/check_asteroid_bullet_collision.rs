use avian2d::prelude::*;
use bevy::prelude::*;

use crate::game::{
    asteroids::asteroid_component::Asteroid, score::score_plugin::Score,
    spaceship::spaceship_plugin::Bullet,
};

// Checks if a bullet and asteroid collided, despawns both, and updates the score.
pub fn check_collision_asteroid_with_bullet(
    mut events: MessageReader<CollisionStart>,
    bullets: Query<(), With<Bullet>>,
    asteroids: Query<(), With<Asteroid>>,
    mut commands: Commands,
    mut score: ResMut<Score>,
) {
    for event in events.read() {
        let entity1 = event.collider1;
        let entity2 = event.collider2;

        if (bullets.contains(entity1) && asteroids.contains(entity2))
            || (bullets.contains(entity2) && asteroids.contains(entity1))
        {
            commands.entity(entity2).despawn();
            commands.entity(entity1).despawn();
            score.increment_score();
        }
    }
}

use avian2d::prelude::*;
use bevy::{platform::collections::HashSet, prelude::*};

use crate::game::{
    asteroids::{asteroid_component::Asteroid, asteroid_plugin::spawn_asteroid},
    score::score_plugin::Score,
    spaceship::spaceship_plugin::Bullet,
};

// Checks if a bullet and asteroid collided, despawns both, and updates the score.
pub fn check_collision_asteroid_with_bullet(
    mut events: MessageReader<CollisionStart>,
    bullets: Query<(), With<Bullet>>,
    asteroids: Query<(&Transform, &Asteroid), With<Asteroid>>,
    mut commands: Commands,
    assets_server: Res<AssetServer>,
    mut score: ResMut<Score>,
    window_s: Single<&Window>,
) {
    let mut processed_asteroid: HashSet<Entity> = HashSet::new();
    for event in events.read() {
        let entity1 = event.collider1;
        let entity2 = event.collider2;

        let (bullet_entity, asteroid_entity) =
            if bullets.contains(entity1) && asteroids.contains(entity2) {
                (entity1, entity2)
            } else if bullets.contains(entity2) && asteroids.contains(entity1) {
                (entity2, entity1)
            } else {
                continue;
            };

        if !processed_asteroid.insert(asteroid_entity) {
            continue;
        }

        let Ok((old_asteroid_transform, old_asteroid)) = asteroids.get(asteroid_entity) else {
            continue;
        };

        if let Some(smaller_asteroid_type) = old_asteroid.asteroid_type.next_asteroid_type() {
            for _ in 0..2 {
                let new_asteroid = Asteroid::new(
                    window_s.width(),
                    window_s.height(),
                    smaller_asteroid_type.clone(),
                );
                let vel = Asteroid::rand_vel();

                spawn_asteroid(
                    &mut commands,
                    &assets_server,
                    old_asteroid_transform.translation,
                    vel,
                    new_asteroid,
                );
            }
        }

        commands.entity(bullet_entity).despawn();
        commands.entity(asteroid_entity).despawn();
        score.increment_score();
    }
}

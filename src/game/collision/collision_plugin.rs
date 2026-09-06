use bevy::prelude::*;

use crate::game::{
    collision::{
        check_asteroid_bullet_collision::{ExplosionMusic, check_collision_asteroid_with_bullet},
        check_asteroid_spaceship_collision::{
            LifeLostSoundEffect, check_asteroid_spaceship_collision,
        },
    },
    game_plugin::GameSet,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ExplosionMusic>()
            .init_resource::<LifeLostSoundEffect>()
            .add_systems(
                Update,
                (
                    check_collision_asteroid_with_bullet,
                    check_asteroid_spaceship_collision,
                )
                    .in_set(GameSet),
            );
    }
}

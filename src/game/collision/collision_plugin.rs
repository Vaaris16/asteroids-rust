use bevy::prelude::*;

use crate::game::collision::{
    check_asteroid_bullet_collision::check_collision_asteroid_with_bullet,
    check_asteroid_spaceship_collision::check_asteroid_spaceship_collision,
};

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, check_collision_asteroid_with_bullet)
            .add_systems(Update, check_asteroid_spaceship_collision);
    }
}

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    GameState,
    game::{asteroids::asteroid_component::Asteroid, spaceship::spaceship_plugin::SpaceShip},
};

pub fn check_asteroid_spaceship_collision(
    asteroids: Query<(), With<Asteroid>>,
    space_ship: Query<(), With<SpaceShip>>,
    mut events: MessageReader<CollisionStart>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for event in events.read() {
        let entity1 = event.collider1;
        let entity2 = event.collider2;

        if (asteroids.contains(entity1) && space_ship.contains(entity2))
            || (asteroids.contains(entity2) && space_ship.contains(entity1))
        {
            next_state.set(GameState::SplashScreen);
        }
    }
}

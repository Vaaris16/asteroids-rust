use bevy::prelude::*;

use crate::game::{
    asteroids::asteroid_plugin::AsteroidPlugin, spaceship::spaceship_plugin::SpaceShipPlugin,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((AsteroidPlugin, SpaceShipPlugin));
    }
}

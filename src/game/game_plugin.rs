use bevy::prelude::*;

use crate::game::spaceship::spaceship_plugin::SpaceShipPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpaceShipPlugin));
    }
}

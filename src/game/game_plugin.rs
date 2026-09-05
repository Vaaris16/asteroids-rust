use bevy::prelude::*;

use crate::{
    GameState,
    game::{
        asteroids::asteroid_plugin::AsteroidPlugin, collision::collision_plugin::CollisionPlugin,
        spaceship::spaceship_plugin::SpaceShipPlugin, ui::ui_plugin::UiPlugin,
    },
};

pub struct GamePlugin;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct GameSet;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, GameSet.run_if(in_state(GameState::Game)));
        app.add_plugins((AsteroidPlugin, SpaceShipPlugin, CollisionPlugin, UiPlugin));
    }
}

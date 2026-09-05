use bevy::prelude::*;

use crate::game::ui::{
    score::score_plugin::ScorePlugin, spaceship_lifes::spaceship_lifes_plugin::SpaceShipLifesPlugin,
};

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ScorePlugin, SpaceShipLifesPlugin));
    }
}

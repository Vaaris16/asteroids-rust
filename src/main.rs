#![allow(warnings)]
use bevy::prelude::*;

use crate::{
    background::background_plugin::BackgroundPlugin, game::game_plugin::GamePlugin,
    player::player_plugin::PlayerPlugin,
};

mod background;
mod game;
mod player;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    resizable: false,
                    ..Default::default()
                }),
                ..Default::default()
            }),
            BackgroundPlugin,
            PlayerPlugin,
            GamePlugin,
        ))
        .run();
}

#![allow(warnings)]
use bevy::prelude::*;

use crate::{
    background::background_plugin::BackgroundPlugin, game::game_plugin::GamePlugin,
    player::player_plugin::PlayerPlugin, splashscreen::splash_screen_plugin::SplashScreenPlugin,
};

mod background;
mod game;
mod player;
mod splashscreen;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

#[derive(Default, States, Hash, Eq, Debug, PartialEq, Clone)]
pub enum GameState {
    #[default]
    SplashScreen,
    Game,
    Retry,
}

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
            SplashScreenPlugin,
        ))
        .init_state::<GameState>()
        .run();
}

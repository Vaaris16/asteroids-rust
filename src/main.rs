use avian2d::PhysicsPlugins;
use bevy::prelude::*;

use crate::{
    core::{background::background_plugin::BackgroundPlugin, player::player_plugin::PlayerPlugin},
    game::game_plugin::GamePlugin,
    retry::retry_plugin::RetryPlugin,
    splashscreen::splash_screen_plugin::SplashScreenPlugin,
};

mod core;
mod game;
mod retry;
mod splashscreen;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

pub const TEXT_COLOR: Color = Color::WHITE;
pub const BORDER_COLOR: Color = Color::WHITE;
pub const FOCUS_TEXT_COLOR: Color = Color::hsl(0., 0., 0.64);
pub const FOCUS_BORDER_COLOR: Color = Color::hsl(0., 0., 0.64);

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
            PhysicsPlugins::default(),
            BackgroundPlugin,
            PlayerPlugin,
            GamePlugin,
            SplashScreenPlugin,
            RetryPlugin,
        ))
        .init_state::<GameState>()
        .run();
}

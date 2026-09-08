use avian2d::PhysicsPlugins;
use bevy::prelude::*;

use crate::{
    core::core_plugin::CorePlugin, game::game_plugin::GamePlugin, retry::retry_plugin::RetryPlugin,
    splashscreen::splash_screen_plugin::SplashScreenPlugin,
};

mod core;
mod game;
mod retry;
mod splashscreen;

pub const BACKGROUND_COLOR: Color = Color::BLACK;

pub const TEXT_COLOR: Color = Color::WHITE;
pub const BORDER_COLOR: Color = Color::WHITE;

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
            CorePlugin,
            GamePlugin,
            SplashScreenPlugin,
            RetryPlugin,
        ))
        .init_state::<GameState>()
        .run();
}

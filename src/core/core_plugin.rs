use crate::core::{
    background::background_plugin::BackgroundPlugin,
    background_music::background_music_plugin::BackgroundMusicPlugin,
    game_assets::game_assets::GameAssets, player::player_plugin::PlayerPlugin,
};
use bevy::prelude::*;

pub struct CorePlugin;

impl Plugin for CorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GameAssets>().add_plugins((
            BackgroundPlugin,
            BackgroundMusicPlugin,
            PlayerPlugin,
        ));
    }
}

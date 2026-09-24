use bevy::prelude::*;

use crate::core::game_assets::game_assets::GameAssets;

pub struct BackgroundMusicPlugin;

impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, play_background_music);
    }
}

fn play_background_music(game_assets: Res<GameAssets>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(game_assets.bg_sound.clone()),
        PlaybackSettings::LOOP,
    ));
}

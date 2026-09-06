use bevy::prelude::*;

pub struct BackgroundMusicPlugin;

#[derive(Resource, Deref)]
struct BackgroundMusic {
    background_music_handle: Handle<AudioSource>,
}

impl FromWorld for BackgroundMusic {
    fn from_world(world: &mut World) -> Self {
        let assets_server = world.resource::<AssetServer>();

        BackgroundMusic {
            background_music_handle: assets_server.load("sounds/background_music.wav"),
        }
    }
}

impl Plugin for BackgroundMusicPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BackgroundMusic>()
            .add_systems(Startup, play_background_music);
    }
}

fn play_background_music(bg_music: Res<BackgroundMusic>, mut commands: Commands) {
    commands.spawn((
        AudioPlayer::new(bg_music.background_music_handle.clone()),
        PlaybackSettings::LOOP,
    ));
}

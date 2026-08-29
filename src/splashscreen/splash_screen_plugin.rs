use crate::{GameState, game_fonts::game_fonts::GameFonts};
use bevy::prelude::*;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), spawn_splash_title);
    }
}

fn spawn_splash_title(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn(Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        })
        .with_children(|splash_title_parent| {
            splash_title_parent.spawn((
                Text::new("ASTEROID"),
                TextFont {
                    font_size: FontSize::Px(150.),
                    font: assets_server
                        .load(GameFonts::ComfortaaBold.font_path())
                        .into(),
                    ..Default::default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

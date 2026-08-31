use bevy::{core_pipeline::oit::resolve::node, prelude::*};

use crate::{
    GameState, TEXT_COLOR, core::game_fonts::game_fonts::GameFonts, game::game_plugin::GameSet,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { score: 0 });
        app.add_systems(OnEnter(GameState::Game), spawn_score)
            .add_systems(Update, set_score_text.in_set(GameSet))
            .add_systems(OnExit(GameState::Game), cleanup_score);
    }
}

const SCORE_STEP: i32 = 5;
const SCORE_TEXT_SIZE: f32 = 100.;

// Component used to identitfy the score text.
#[derive(Component)]
struct ScoreText;

// Resource that stores the player's current score.
#[derive(Resource)]
pub struct Score {
    pub score: i32,
}

impl Score {
    // increments the score by the SCORE_STEP
    pub fn increment_score(&mut self) {
        self.score += SCORE_STEP;
    }
}

fn spawn_score(mut commands: Commands, score: Res<Score>, assets_server: Res<AssetServer>) {
    commands
        .spawn(
            (Node {
                width: percent(100),
                height: percent(100),
                justify_content: JustifyContent::Center,
                ..Default::default()
            }),
        )
        .with_children(|score_parent| {
            score_parent.spawn((
                Text::new(score.score.to_string()),
                TextFont {
                    font_size: px(SCORE_TEXT_SIZE).into(),
                    font: assets_server
                        .load(GameFonts::ComfortaaBold.font_path())
                        .into(),
                    ..Default::default()
                },
                TextColor(TEXT_COLOR),
                ScoreText,
            ));
        });
}

// Updates the score text with the current Score
fn set_score_text(mut score_text: Single<&mut Text, With<ScoreText>>, score: ResMut<Score>) {
    score_text.0 = score.score.to_string();
}

// Removes score text.
fn cleanup_score(mut commands: Commands, score_text: Single<Entity, With<ScoreText>>) {
    commands.entity(*score_text).despawn();
}

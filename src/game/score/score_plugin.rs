use bevy::{core_pipeline::oit::resolve::node, prelude::*};

use crate::{GameState, game::game_plugin::GameSet};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Score { score: 0 });
        app.add_systems(OnEnter(GameState::Game), spawn_score)
            .add_systems(Update, set_score_text.in_set(GameSet));
    }
}

const SCORE_STEP: i32 = 5;

// Component used to identitfy the score text.
#[derive(Component)]
struct ScoreText;

// Resource that stores the player's current score.
#[derive(Resource)]
pub struct Score {
    score: i32,
}

impl Score {
    // increments the score by the SCORE_STEP
    pub fn increment_score(&mut self) {
        self.score += SCORE_STEP;
    }
}

fn spawn_score(mut commands: Commands, score: Res<Score>) {
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
                    font_size: FontSize::Px(100.),
                    ..Default::default()
                },
                TextColor(Color::WHITE),
                ScoreText,
            ));
        });
}

// Updates the score text with the current Score
fn set_score_text(mut score_text: Single<&mut Text, With<ScoreText>>, score: ResMut<Score>) {
    score_text.0 = score.score.to_string();
}

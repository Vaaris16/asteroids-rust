use bevy::{log::tracing_subscriber::fmt::format, prelude::*};

use crate::{
    BORDER_COLOR,
    GameState::{self, Game},
    TEXT_COLOR,
    core::game_fonts::game_fonts::GameFonts,
    game::score::score_plugin::Score,
};

pub struct RetryPlugin;

#[derive(SystemSet, PartialEq, Eq, Debug, Hash, Clone)]
struct RetrySet;

impl Plugin for RetryPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, RetrySet.run_if(in_state(GameState::Retry)));
        app.add_systems(OnEnter(GameState::Retry), retry_window)
            .add_systems(Update, retry_button_interactions.in_set(RetrySet))
            .add_systems(OnExit(GameState::Retry), clean_up_retry)
            .add_systems(OnExit(GameState::Retry), reset_score);
    }
}

#[derive(Component)]
struct RetryPage;

#[derive(Component)]
struct RetryButton;

// Spawns the main retry page.
fn retry_window(mut commands: Commands, assets_server: Res<AssetServer>, score: Res<Score>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        RetryPage,
        children![modal_window(&assets_server, score)],
    ));
}

// Spawns the retry modal window.
fn modal_window(assets_server: &AssetServer, score: Res<Score>) -> impl Bundle {
    (
        Node {
            width: percent(33),
            height: percent(70),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(px(2)),
            border_radius: BorderRadius::all(px(20)),
            padding: UiRect::all(px(50)),
            ..Default::default()
        },
        BorderColor::all(BORDER_COLOR),
        BackgroundColor(Color::BLACK),
        children![
            score_title(assets_server),
            final_score(score),
            retry_button(&assets_server)
        ],
    )
}

// Spawns the score title.
fn score_title(assets_server: &AssetServer) -> impl Bundle {
    (
        Text::new("Score"),
        TextFont {
            font_size: px(75).into(),
            font: assets_server
                .load(GameFonts::ComfortaaBold.font_path())
                .into(),
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
    )
}

// Displays the final score.
fn final_score(score: Res<Score>) -> impl Bundle {
    (
        Node {
            margin: UiRect::top(px(40)),
            ..Default::default()
        },
        Text::new(score.score.to_string()),
        TextColor(TEXT_COLOR),
        TextFont {
            font_size: px(100).into(),
            ..Default::default()
        },
    )
}

// Spawns the retry button.
fn retry_button(assets_server: &AssetServer) -> impl Bundle {
    (
        Button,
        Node {
            width: px(200),
            height: px(80),
            border_radius: BorderRadius::all(px(20.)),
            margin: UiRect::top(px(50)),
            border: UiRect::all(px(2.5)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
        BorderColor::all(BORDER_COLOR),
        RetryButton,
        children![(
            Text::new("Retry"),
            TextFont {
                font_size: px(40).into(),
                font: assets_server
                    .load(GameFonts::ComfortaaBold.font_path())
                    .into(),
                ..Default::default()
            },
            TextColor(TEXT_COLOR),
        )],
    )
}

// Handles interactions with the retry button.
fn retry_button_interactions(
    mut game_state: ResMut<NextState<GameState>>,
    interaction_query: Query<(&Interaction, &Button), (With<RetryButton>, Changed<Interaction>)>,
) {
    for (interaction, button) in interaction_query {
        match *interaction {
            Interaction::Pressed => {
                game_state.set(GameState::SplashScreen);
            }
            _ => (),
        }
    }
}

// Cleans up the retry page.
fn clean_up_retry(mut commands: Commands, retry_page: Single<Entity, With<RetryPage>>) {
    commands.entity(*retry_page).despawn();
}

// Resets the score.
fn reset_score(mut score: ResMut<Score>) {
    score.score = 0;
}

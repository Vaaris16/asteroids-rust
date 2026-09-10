use bevy::prelude::*;

use crate::{
    BORDER_COLOR,
    GameState::{self},
    TEXT_COLOR,
    core::game_fonts::fonts::GameFonts,
    game::ui::score::score_plugin::Score,
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

#[derive(Component)]
struct RetryButtonText;

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

// Width and height of the retry modal as a percentage of the screen.
const MODAL_WINDOW_WIDTH: f32 = 33.;
// Border thickness of the retry modal window.
const MODAL_WINDOW_BORDER_THICKNESS: f32 = 2.;

// Spawns the retry modal window.
fn modal_window(assets_server: &AssetServer, score: Res<Score>) -> impl Bundle {
    (
        Node {
            width: percent(MODAL_WINDOW_WIDTH),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(px(MODAL_WINDOW_BORDER_THICKNESS)),
            padding: UiRect {
                top: px(50),
                bottom: px(50),
                ..Default::default()
            },
            ..Default::default()
        },
        BorderColor::all(BORDER_COLOR),
        BackgroundColor(Color::BLACK),
        children![
            game_over_text(assets_server),
            score_title(assets_server),
            final_score(score, assets_server),
            retry_button(assets_server)
        ],
    )
}

fn game_over_text(assets_server: &AssetServer) -> impl Bundle {
    (
        Text::new("GAME OVER"),
        TextFont {
            font: assets_server
                .load(GameFonts::PressStart2P.font_path())
                .into(),
            font_size: px(35).into(),
            ..Default::default()
        },
    )
}

// Font size for the "Score" title.
const SCORE_TITLE_FONT_SIZE: f32 = 30.;

// Spawns the score title.
fn score_title(assets_server: &AssetServer) -> impl Bundle {
    (
        Node {
            margin: UiRect::top(px(30)),
            ..Default::default()
        },
        Text::new("Final Score"),
        TextFont {
            font_size: px(SCORE_TITLE_FONT_SIZE).into(),
            font: assets_server
                .load(GameFonts::PressStart2P.font_path())
                .into(),
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
    )
}

// Font size for the player's final score.
const FINAL_SCORE_FONT_SIZE: f32 = 50.;

// Displays the final score.
fn final_score(score: Res<Score>, assets_server: &AssetServer) -> impl Bundle {
    (
        Node {
            margin: UiRect::top(px(20)),
            ..Default::default()
        },
        Text::new(score.score.to_string()),
        TextColor(TEXT_COLOR),
        TextFont {
            font: assets_server
                .load(GameFonts::PressStart2P.font_path())
                .into(),
            font_size: px(FINAL_SCORE_FONT_SIZE).into(),
            ..Default::default()
        },
    )
}

// Font size for the retry button text.
const RETRY_BUTTON_TEXT_FONT_SIZE: f32 = 25.;
// Width and height of the retry button in pixels.
const RETRY_BUTTON_DIMENSIONS: Vec2 = Vec2::new(200., 70.);
// Border thickness of the retry button.
const RETRY_BUTTON_BORDER_THICKNESS: f32 = 2.5;

// Spawns the retry button.
fn retry_button(assets_server: &AssetServer) -> impl Bundle {
    (
        Button,
        Node {
            width: px(RETRY_BUTTON_DIMENSIONS.x),
            height: px(RETRY_BUTTON_DIMENSIONS.y),
            margin: UiRect::top(px(50)),
            border: UiRect::all(px(RETRY_BUTTON_BORDER_THICKNESS)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BackgroundColor(Color::BLACK),
        BorderColor::all(BORDER_COLOR),
        RetryButton,
        children![(
            RetryButtonText,
            Text::new("Retry"),
            TextFont {
                font_size: px(RETRY_BUTTON_TEXT_FONT_SIZE).into(),
                font: assets_server
                    .load(GameFonts::PressStart2P.font_path())
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
    retry_button: Query<
        (&Interaction, &mut BackgroundColor),
        (With<RetryButton>, Changed<Interaction>),
    >,
    mut retry_button_text: Single<&mut TextColor, With<RetryButtonText>>,
) {
    for (interaction, mut bg_color) in retry_button {
        match *interaction {
            Interaction::Pressed => {
                game_state.set(GameState::SplashScreen);
            }
            Interaction::Hovered => {
                retry_button_text.0 = Color::BLACK;
                bg_color.0 = Color::WHITE;
            }
            Interaction::None => {
                retry_button_text.0 = Color::WHITE;
                bg_color.0 = Color::BLACK;
            }
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

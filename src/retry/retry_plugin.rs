use bevy::prelude::*;

use crate::{
    BORDER_COLOR, FOCUS_BORDER_COLOR, FOCUS_TEXT_COLOR,
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

// Font size for the "Score" title.
const SCORE_TITLE_FONT_SIZE: f32 = 75.;

// Font size for the player's final score.
const FINAL_SCORE_FONT_SIZE: f32 = 100.;

// Font size for the retry button text.
const RETRY_BUTTON_TEXT_FONT_SIZE: f32 = 40.;

// Width and height of the retry modal as a percentage of the screen.
const MODAL_WINDOW_DIMENSIONS_PERCENT: Vec2 = Vec2::new(33., 70.);

// Corner radius of the retry modal window.
const MODAL_WINDOW_BORDER_RADIUS: f32 = 20.;

// Border thickness of the retry modal window.
const MODAL_WINDOW_BORDER_THICKNESS: f32 = 2.;

// Text displayed above the final score.
const SCORE_TITLE: &str = "Score";

// Width and height of the retry button in pixels.
const RETRY_BUTTON_DIMENSIONS: Vec2 = Vec2::new(200., 80.);

// Corner radius of the retry button.
const RETRY_BUTTON_BORDER_RADIUS: f32 = 20.;

// Border thickness of the retry button.
const RETRY_BUTTON_BORDER_THICKNESS: f32 = 2.5;

// Text displayed inside the retry button.
const RETRY_BUTTON_TEXT: &str = "retry";

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

// Spawns the retry modal window.
fn modal_window(assets_server: &AssetServer, score: Res<Score>) -> impl Bundle {
    (
        Node {
            width: percent(MODAL_WINDOW_DIMENSIONS_PERCENT.x),
            height: percent(MODAL_WINDOW_DIMENSIONS_PERCENT.y),
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            border: UiRect::all(px(MODAL_WINDOW_BORDER_THICKNESS)),
            border_radius: BorderRadius::all(px(MODAL_WINDOW_BORDER_RADIUS)),
            padding: UiRect::all(px(50)),
            ..Default::default()
        },
        BorderColor::all(BORDER_COLOR),
        BackgroundColor(Color::BLACK),
        children![
            score_title(assets_server),
            final_score(score),
            retry_button(assets_server)
        ],
    )
}

// Spawns the score title.
fn score_title(assets_server: &AssetServer) -> impl Bundle {
    (
        Text::new(SCORE_TITLE),
        TextFont {
            font_size: px(SCORE_TITLE_FONT_SIZE).into(),
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
            font_size: px(FINAL_SCORE_FONT_SIZE).into(),
            ..Default::default()
        },
    )
}

// Spawns the retry button.
fn retry_button(assets_server: &AssetServer) -> impl Bundle {
    (
        Button,
        Node {
            width: px(RETRY_BUTTON_DIMENSIONS.x),
            height: px(RETRY_BUTTON_DIMENSIONS.y),
            border_radius: BorderRadius::all(px(RETRY_BUTTON_BORDER_RADIUS)),
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
            Text::new(RETRY_BUTTON_TEXT),
            TextFont {
                font_size: px(RETRY_BUTTON_TEXT_FONT_SIZE).into(),
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
    retry_button: Query<
        (&Interaction, &mut BorderColor),
        (With<RetryButton>, Changed<Interaction>),
    >,
    mut retry_button_text: Single<&mut TextColor, With<RetryButtonText>>,
) {
    for (interaction, mut border_color) in retry_button {
        match *interaction {
            Interaction::Pressed => {
                game_state.set(GameState::SplashScreen);
            }
            Interaction::Hovered => {
                retry_button_text.0 = FOCUS_TEXT_COLOR;
                *border_color = BorderColor::all(FOCUS_BORDER_COLOR);
            }
            Interaction::None => {
                retry_button_text.0 = TEXT_COLOR;
                *border_color = BorderColor::all(BORDER_COLOR);
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

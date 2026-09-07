use crate::{
    BORDER_COLOR,
    GameState::{self},
    TEXT_COLOR,
    core::game_fonts::fonts::GameFonts,
};
use bevy::prelude::*;

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), splash_screen)
            .add_systems(Update, button_interactions)
            .add_systems(OnExit(GameState::SplashScreen), clean_up_splash_screen);
    }
}

// Font size of the splash screen title.
const FONT_SIZE_SPLASH_TITLE: i32 = 120;

// Font size of the start button text.
const START_BUTTON_FONT_SIZE: i32 = 20;

// Text displayed as the splash screen title.
const SPLASH_TITLE: &str = "ASTEROID";

// Text displayed inside the start button.
const START_BUTTON_TEXT: &str = "START";

// Width and height of the start button in pixels.
const SPLASH_START_BUTTON_SIZE: Vec2 = Vec2::new(250., 60.);

// Border thickness of the start button.
const START_BUTTON_BORDER_THICKNESS: f32 = 2.5;

#[derive(Component)]
struct SplashScreenComponent;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct StartButtonText;

// Spawns the splash screen.
fn splash_screen(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: px(50),
            ..Default::default()
        },
        SplashScreenComponent,
        children![splash_title(&assets_server), start_button(&assets_server)],
    ));
}

// Spawns the splash screen title.
fn splash_title(assets_server: &AssetServer) -> impl Bundle {
    (
        Text::new(SPLASH_TITLE),
        TextFont {
            font_size: px(FONT_SIZE_SPLASH_TITLE).into(),
            font: assets_server
                .load(GameFonts::PressStart2P.font_path())
                .into(),
            ..Default::default()
        },
        TextColor(TEXT_COLOR),
    )
}

// Spawns the start button
fn start_button(assets_server: &AssetServer) -> impl Bundle {
    (
        Button,
        Node {
            width: px(SPLASH_START_BUTTON_SIZE.x),
            height: px(SPLASH_START_BUTTON_SIZE.y),
            border: UiRect::all(px(START_BUTTON_BORDER_THICKNESS)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..Default::default()
        },
        BorderColor::all(BORDER_COLOR),
        BackgroundColor(Color::BLACK),
        StartButton,
        children![(
            StartButtonText,
            Text::new(START_BUTTON_TEXT),
            TextColor(TEXT_COLOR),
            TextFont {
                font: assets_server
                    .load(GameFonts::PressStart2P.font_path())
                    .into(),
                font_size: px(START_BUTTON_FONT_SIZE).into(),
                ..Default::default()
            },
        )],
    )
}

// Handles interactions with the start button
fn button_interactions(
    mut game_state: ResMut<NextState<GameState>>,
    start_button: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<StartButton>),
    >,
    mut start_button_text_color: Single<&mut TextColor, With<StartButtonText>>,
) {
    for (interactions, mut background_color) in start_button {
        match *interactions {
            Interaction::Pressed => {
                game_state.set(GameState::Game);
            }
            Interaction::Hovered => {
                background_color.0 = Color::WHITE;
                start_button_text_color.0 = Color::BLACK;
            }
            Interaction::None => {
                background_color.0 = Color::BLACK;
                start_button_text_color.0 = Color::WHITE;
            }
        }
    }
}

// Cleans up the splash screen
fn clean_up_splash_screen(
    mut commands: Commands,
    splash_screen_entity: Single<Entity, With<SplashScreenComponent>>,
) {
    commands.entity(*splash_screen_entity).despawn();
}

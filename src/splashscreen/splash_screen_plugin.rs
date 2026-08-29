use crate::{
    GameState::{self, SplashScreen},
    core::game_fonts::game_fonts::GameFonts,
};
use bevy::{input::gamepad::GamepadButton::Start, prelude::*};

pub struct SplashScreenPlugin;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::SplashScreen), splash_screen)
            .add_systems(Update, button_interactions)
            .add_systems(OnExit(GameState::SplashScreen), clean_up_splash_screen);
    }
}

#[derive(Component)]
struct SplashScreenComponent;

#[derive(Component)]
struct StartButton;

fn splash_screen(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands.spawn((
        Node {
            width: percent(100),
            height: percent(100),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            column_gap: px(35),
            ..Default::default()
        },
        SplashScreenComponent,
        children![splash_title(&assets_server), start_button(&assets_server)],
    ));
}

fn splash_title(assets_server: &AssetServer) -> impl Bundle {
    (
        Text::new("ASTEROID"),
        TextFont {
            font_size: px(150).into(),
            font: assets_server
                .load(GameFonts::ComfortaaBold.font_path())
                .into(),
            ..Default::default()
        },
        TextColor(Color::WHITE),
    )
}

fn start_button(assets_server: &AssetServer) -> impl Bundle {
    (
        Button,
        Node {
            width: px(200),
            height: px(75.),
            border: UiRect::all(px(2.5)),
            border_radius: BorderRadius::all(px(10)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            padding: UiRect::all(Val::Px(1.0)),
            ..Default::default()
        },
        BorderColor::all(Color::WHITE),
        BackgroundColor(Color::BLACK),
        StartButton,
        children![(
            Text::new("START"),
            TextColor(Color::WHITE),
            TextFont {
                font: assets_server
                    .load(GameFonts::ComfortaaMedium.font_path())
                    .into(),
                font_size: px(30).into(),
                ..Default::default()
            },
        )],
    )
}

fn button_interactions(
    mut game_state: ResMut<NextState<GameState>>,
    interaction_query: Query<
        (&mut Button, &Interaction),
        (Changed<Interaction>, With<StartButton>),
    >,
) {
    for (button, interactions) in interaction_query {
        match *interactions {
            Interaction::Pressed => {
                game_state.set(GameState::Game);
            }
            _ => (),
        }
    }
}

fn clean_up_splash_screen(
    mut commands: Commands,
    splash_screen_entity: Single<Entity, With<SplashScreenComponent>>,
) {
    commands.entity(*splash_screen_entity).despawn();
}

use bevy::prelude::*;

use crate::{GameState, game::spaceship::spaceship_plugin::SPACE_SHIP_IMAGE_PATH};

pub struct SpaceShipLifesPlugin;

#[derive(Resource)]
pub struct SpaceShipLifes {
    remaining_lifes: i32,
}

#[derive(Component)]
pub struct SpaceShipLifeUi;

#[derive(Component)]
pub struct SpaceShipLifesIndicator {
    index: i32,
}

impl Plugin for SpaceShipLifesPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(SpaceShipLifes { remaining_lifes: 3 })
            .add_systems(OnEnter(GameState::Game), spaceship_lifes)
            .add_systems(
                OnExit(GameState::Game),
                (cleanup_spaceship_lifes_ui, reset_spaceship_lifes),
            );
    }
}

pub fn decrement_lifes(
    life_indicator: Query<(Entity, &SpaceShipLifesIndicator), With<SpaceShipLifesIndicator>>,
    lifes: &mut SpaceShipLifes,
    game_state: &mut NextState<GameState>,
    commands: &mut Commands,
) {
    lifes.remaining_lifes -= 1;

    if lifes.remaining_lifes == 0 {
        game_state.set(GameState::Retry);
    }

    for (indicator_entity, indicator) in life_indicator {
        if indicator.index > lifes.remaining_lifes {
            commands.entity(indicator_entity).despawn();
        }
    }

    println!("{}", lifes.remaining_lifes);
}

fn spaceship_lifes(mut commands: Commands, assets_server: Res<AssetServer>) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: px(20),
                right: px(50),
                column_gap: px(30),
                justify_content: JustifyContent::FlexEnd,
                ..Default::default()
            },
            SpaceShipLifeUi,
        ))
        .with_children(|life| {
            for i in 1..4 {
                life.spawn((
                    ImageNode::new(assets_server.load(SPACE_SHIP_IMAGE_PATH)),
                    SpaceShipLifesIndicator { index: i },
                    Node {
                        width: px(40),
                        height: px(50),
                        ..Default::default()
                    },
                ));
            }
        });
}

fn reset_spaceship_lifes(mut lifes: ResMut<SpaceShipLifes>) {
    lifes.remaining_lifes = 3;
}

fn cleanup_spaceship_lifes_ui(mut commands: Commands, ui: Single<Entity, With<SpaceShipLifeUi>>) {
    commands.entity(*ui).despawn();
}

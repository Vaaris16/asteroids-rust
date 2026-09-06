use avian2d::prelude::*;
use bevy::prelude::*;

use crate::{
    GameState,
    game::{
        asteroids::asteroid_component::Asteroid,
        spaceship::spaceship_plugin::SpaceShip,
        ui::spaceship_lifes::spaceship_lifes_plugin::{
            SpaceShipLifes, SpaceShipLifesIndicator, decrement_lifes,
        },
    },
};

#[derive(Resource, Deref)]
pub struct LifeLostSoundEffect {
    life_lost_sound_effect_handle: Handle<AudioSource>,
}

impl FromWorld for LifeLostSoundEffect {
    fn from_world(world: &mut World) -> Self {
        let assets_server = world.resource::<AssetServer>();
        LifeLostSoundEffect {
            life_lost_sound_effect_handle: assets_server.load("sounds/lose_life_sound_effect.wav"),
        }
    }
}

pub fn check_asteroid_spaceship_collision(
    asteroids: Query<(), With<Asteroid>>,
    space_ship: Query<(), With<SpaceShip>>,
    mut events: MessageReader<CollisionStart>,
    mut next_state: ResMut<NextState<GameState>>,
    mut lifes: ResMut<SpaceShipLifes>,
    life_indicator: Query<(Entity, &SpaceShipLifesIndicator), With<SpaceShipLifesIndicator>>,
    mut commands: Commands,
    sound_effect: Res<LifeLostSoundEffect>,
) {
    for event in events.read() {
        let entity1 = event.collider1;
        let entity2 = event.collider2;

        if (asteroids.contains(entity1) && space_ship.contains(entity2))
            || (asteroids.contains(entity2) && space_ship.contains(entity1))
        {
            decrement_lifes(life_indicator, &mut lifes, &mut next_state, &mut commands);
            commands.spawn((
                AudioPlayer::new(sound_effect.life_lost_sound_effect_handle.clone()),
                PlaybackSettings::DESPAWN,
            ));
        }
    }
}

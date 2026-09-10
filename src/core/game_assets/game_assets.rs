use bevy::prelude::*;
use rand::RngExt;

#[derive(Resource, Clone)]
pub struct GameAssets {
    pub spaceship_image: Handle<Image>,
    pub bullet_image: Handle<Image>,
    pub asteroid_1: Handle<Image>,
    pub asteroid_2: Handle<Image>,
    pub asteroid_3: Handle<Image>,

    pub explosion_sound: Handle<AudioSource>,
    pub shoot_sound: Handle<AudioSource>,
    pub bg_sound: Handle<AudioSource>,
    pub life_decrement_sound: Handle<AudioSource>,
}

// Image path of the space ship.
const SPACE_SHIP_IMAGE_PATH: &str = "space_ship.png";
// Image path of the bullet.
const BULLET_IMAGE_PATH: &str = "bullet.png";
// Defines the image paths for the asteroids.
const ASTEROID_PATH_1: &str = "asteroids_images/asteroid_1.png";
const ASTEROID_PATH_2: &str = "asteroids_images/asteroid_2.png";
const ASTEROID_PATH_3: &str = "asteroids_images/asteroid_3.png";

// Sound effects.
const EXPLOSION_SOUND_EFFECT: &str = "sounds/explosion.wav";
const SHOOT_SOUND_EFFECT: &str = "sounds/shoot_sound.wav";
const BACKGROUND_MUSIC_PATH: &str = "sounds/background_music.wav";
const LOSE_LIFE_SOUND_PATH: &str = "sounds/lose_life_sound_effect.wav";

impl FromWorld for GameAssets {
    fn from_world(world: &mut World) -> Self {
        let assets_server = world.resource::<AssetServer>();

        GameAssets {
            spaceship_image: assets_server.load(SPACE_SHIP_IMAGE_PATH),
            bullet_image: assets_server.load(BULLET_IMAGE_PATH),
            asteroid_1: assets_server.load(ASTEROID_PATH_1),
            asteroid_2: assets_server.load(ASTEROID_PATH_2),
            asteroid_3: assets_server.load(ASTEROID_PATH_3),
            explosion_sound: assets_server.load(EXPLOSION_SOUND_EFFECT),
            shoot_sound: assets_server.load(SHOOT_SOUND_EFFECT),
            bg_sound: assets_server.load(BACKGROUND_MUSIC_PATH),
            life_decrement_sound: assets_server.load(LOSE_LIFE_SOUND_PATH),
        }
    }
}

impl GameAssets {
    pub fn rand_asteroids(&self) -> Handle<Image> {
        let mut rng = rand::rng();

        match rng.random_range(0..3) {
            0 => self.asteroid_1.clone(),
            1 => self.asteroid_2.clone(),
            2 => self.asteroid_3.clone(),
            _ => unreachable!(),
        }
    }
}

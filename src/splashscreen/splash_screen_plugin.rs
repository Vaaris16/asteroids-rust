use bevy::prelude::*;

use crate::GameState;

pub struct SplashScreenPlugin;

#[derive(SystemSet, Hash, Debug, Eq, PartialEq, Clone)]
pub struct SplashScreenSet;

impl Plugin for SplashScreenPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            SplashScreenSet.run_if(in_state(GameState::SplashScreen)),
        );
    }
}

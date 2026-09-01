use bevy::prelude::*;

use crate::BACKGROUND_COLOR;

pub struct BackgroundPlugin;

impl Plugin for BackgroundPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, set_default_bg);
    }
}

fn set_default_bg(mut clear_color: ResMut<ClearColor>) {
    clear_color.0 = BACKGROUND_COLOR;
}

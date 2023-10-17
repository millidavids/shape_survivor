mod components;
mod dot;

use bevy::prelude::*;

use self::dot::DotPlugin;

pub const DEFAULT_ABILITY_SPEED: f32 = 500.0;

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DotPlugin);
    }
}

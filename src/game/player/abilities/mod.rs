mod components;
mod dot;
pub mod events;

use bevy::prelude::*;

use self::{dot::DotPlugin, events::TransmitDamage};

pub const DEFAULT_ABILITY_SPEED: f32 = 500.0;

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TransmitDamage>()
            .add_plugins(DotPlugin);
    }
}

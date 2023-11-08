mod components;
mod systems;
mod styles;

use bevy::prelude::*;

use self::systems::{spawn_xp_bar, animate_xp_bar};

use super::states::GameState;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Running), spawn_xp_bar)
            .add_systems(Update, animate_xp_bar.run_if(in_state(GameState::Running)));
    }
}

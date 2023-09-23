mod components;
mod systems;
mod states;
mod styles;
mod resources;

use bevy::prelude::*;

use crate::{game::states::GameState, states::AppState};

use self::systems::{spawn_pause_menu, despawn_pause_menu, button_interaction};

pub struct PausedPlugin;

impl Plugin for PausedPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Paused), spawn_pause_menu.run_if(in_state(AppState::Game)))
            .add_systems(Update, (button_interaction).run_if(in_state(GameState::Paused)))
            .add_systems(OnExit(GameState::Paused), despawn_pause_menu);
    }
}
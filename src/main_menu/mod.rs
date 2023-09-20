mod components;
mod systems;
mod styles;
mod resources;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{spawn_main_menu, despawn_main_menu, button_interaction};

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
            .add_systems(Update, button_interaction)
            .add_systems(OnExit(AppState::MainMenu), despawn_main_menu);
    }
}
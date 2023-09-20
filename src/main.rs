mod systems;
mod states;
mod main_menu;

use bevy::prelude::*;
use main_menu::MainMenuPlugin;
use systems::{spawn_camera, toggle_game_state};
use states::AppState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, toggle_game_state)
        .run();
}

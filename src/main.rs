mod game;
mod main_menu;
mod states;
mod systems;

use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use states::AppState;
use systems::{spawn_camera, toggle_app_state};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::Hsla {
            hue: 56.0,
            saturation: 0.09,
            lightness: 0.96,
            alpha: 1.0,
        }))
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, toggle_app_state)
        .run();
}

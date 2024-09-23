mod game;
mod main_menu;
mod states;
mod systems;

use bevy::prelude::*;
// use bevy_inspector_egui::quick::WorldInspectorPlugin;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use states::AppState;
use systems::{spawn_camera, toggle_app_state};

fn main() {
    let mut app = App::new();

    app.insert_resource(ClearColor(Color::BLACK))
        .add_plugins(DefaultPlugins)
        .add_state::<AppState>()
        .add_plugins(MainMenuPlugin)
        .add_plugins(GamePlugin)
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, toggle_app_state);

    // if cfg!(debug_assertions) {
    //     app.add_plugins(WorldInspectorPlugin::new());
    // }

    app.run();
}

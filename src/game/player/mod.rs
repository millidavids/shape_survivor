mod systems;
mod components;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::spawn_player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_player);
    }
}
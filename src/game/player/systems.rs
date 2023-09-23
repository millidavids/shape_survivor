use bevy::prelude::*;

use crate::game::components::Game;

use super::components::Player;

pub fn spawn_player(
    mut commands: Commands,
    game_query: Query<Entity, With<Game>>,
) {
    if let Ok(game_entity) = game_query.get_single() {
        commands.entity(game_entity).with_children(|parent| {
            parent.spawn(Player {});
        });
    }
}
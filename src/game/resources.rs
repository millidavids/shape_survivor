use bevy::prelude::*;

#[derive(Resource)]
pub struct GameStatus {
    pub new_game_requested: bool,
    pub game_paused: bool,
}

impl Default for GameStatus {
    fn default() -> Self {
        GameStatus {
            new_game_requested: true,
            game_paused: true,
        }
    }
}

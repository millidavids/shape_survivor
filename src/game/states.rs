use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum GameState {
    #[default]
    Inactive,
    NewGame,
    Paused,
    Running,
    LevelUp,
}

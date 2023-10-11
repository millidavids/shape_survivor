mod components;
mod styles;
mod systems;

use bevy::prelude::*;

use crate::states::AppState;

use self::systems::{despawn_grid, spawn_grid};

pub const GRID_SMALL_BOX_LENGTH: u32 = 32;
pub const GRID_LARGE_BOX_LENGTH: u32 = GRID_SMALL_BOX_LENGTH * 5;

pub const NUM_SMALL_BOX_WIDTH: u32 = 850;
pub const NUM_SMALL_BOX_HEIGHT: u32 = 1100;
pub const NUM_LARGE_BOX_WIDTH: u32 = NUM_SMALL_BOX_WIDTH / 5;
pub const NUM_LARGE_BOX_HEIGHT: u32 = NUM_SMALL_BOX_HEIGHT / 5;

pub const GRID_WIDTH: u32 = GRID_SMALL_BOX_LENGTH * NUM_SMALL_BOX_WIDTH;
pub const GRID_HEIGHT: u32 = GRID_SMALL_BOX_LENGTH * NUM_SMALL_BOX_HEIGHT;

pub struct GridPlugin;

impl Plugin for GridPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Game), spawn_grid)
            .add_systems(OnExit(AppState::Game), despawn_grid);
    }
}

use bevy::prelude::*;

use crate::game::states::GameState;

use super::components::{PauseMenu, PauseMenuButton};
use super::styles::{
    get_button_text, HOVERED_BUTTON_COLOR, NORMAL_BUTTON_COLOR, PAUSE_BUTTON_STYLE,
    PAUSE_MENU_STYLE, PAUSE_MENU_TRANSFORM, PRESSED_BUTTON_COLOR,
};

/// Spawns a pause menu in the game.
///
/// This system creates a pause menu consisting of a background node and two buttons: a "Resume" button and a "Main Menu" button.
/// The pause menu and its buttons use predefined styles and transforms.
///
/// # Parameters:
/// - `mut commands`: A command buffer for creating and modifying entities, resources, and systems.
/// - `asset_server`: The main interface to access assets in Bevy, used here to load text for buttons.
///
/// # Examples:
/// This system can be added to an app's schedule to spawn the pause menu, typically when transitioning to a paused game state:
///
/// ```rust
/// app.add_systems(OnEnter(AppState::Paused), spawn_pause_menu);
/// ```
///
/// Note:
/// The constants `PAUSE_MENU_STYLE`, `PAUSE_MENU_TRANSFORM`, and `PAUSE_BUTTON_STYLE` should be defined elsewhere in the codebase. Additionally, the function `get_button_text` should be a utility function that retrieves a `Text` asset based on a provided label.
///
/// # Child Entities:
/// The system spawns child entities for the pause menu:
/// 1. A "Resume" button with the text "Resume".
/// 2. A "Main Menu" button with the text "Main Menu".
///
/// Both buttons have their own specific functionality tagged by the `PauseMenuButton` enum variant (`Running` and `MainMenu` respectively).
pub fn spawn_pause_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: PAUSE_MENU_STYLE,
                transform: PAUSE_MENU_TRANSFORM,
                ..default()
            },
            PauseMenu {},
        ))
        .with_children(|parent| {
            // ---- Play Button ----
            parent
                .spawn((
                    ButtonBundle {
                        style: PAUSE_BUTTON_STYLE,
                        transform: PAUSE_MENU_TRANSFORM,
                        ..default()
                    },
                    PauseMenuButton::Running,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, "Resume"),
                        ..default()
                    });
                });
            // ---- Quit Button ----
            parent
                .spawn((
                    ButtonBundle {
                        style: PAUSE_BUTTON_STYLE,
                        transform: PAUSE_MENU_TRANSFORM,
                        ..default()
                    },
                    PauseMenuButton::MainMenu,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, "Main Menu"),
                        ..default()
                    });
                });
        });
}

/// Despawns the pause menu from the game.
///
/// This system searches for an entity with the `PauseMenu` component and despawns it, along with its children.
/// This is typically used to remove the pause menu from the scene when resuming gameplay or transitioning to a different game state.
///
/// # Parameters:
/// - `mut commands`: A command buffer for creating and modifying entities, resources, and systems.
/// - `pause_menu_query`: A query to identify entities that have the `PauseMenu` component.
///
/// # Examples:
/// This system can be added to an app's schedule to despawn the pause menu, typically when transitioning from a paused state to a running game state:
///
/// ```rust
/// app.add_systems(OnExit(AppState::Paused), despawn_pause_menu);
/// ```
///
/// # Notes:
/// Ensure that there's only one `PauseMenu` entity in the game at any given time since the system uses `get_single()` which expects a single entity.
pub fn despawn_pause_menu(
    mut commands: Commands,
    pause_menu_query: Query<Entity, With<PauseMenu>>,
) {
    if let Ok(pause_menu_entity) = pause_menu_query.get_single() {
        commands.entity(pause_menu_entity).despawn_recursive();
    }
}

/// Handles button interactions within the pause menu.
///
/// This system checks for button interactions on entities with the `PauseMenuButton` component. Depending on the type of button and the interaction state, it will update the button's background color and may also change the game state.
///
/// # Parameters:
/// - `mut button_query`: A query to identify entities that have an `Interaction`, `BackgroundColor`, and `PauseMenuButton` component. This query also listens for changes to the `Interaction` state.
/// - `mut next_game_state`: A mutable reference to the `NextState<GameState>` resource, allowing this system to change the game state based on button interactions.
///
/// # Interactions:
/// 1. If the `PauseMenuButton::Running` button is pressed, it will:
///     - Change the button's background color to `PRESSED_BUTTON_COLOR`.
///     - Set the next game state to `GameState::Running`.
/// 2. If the `PauseMenuButton::MainMenu` button is pressed, it will:
///     - Change the button's background color to `PRESSED_BUTTON_COLOR`.
///     - Set the next game state to `GameState::Inactive`.
/// 3. If any button is hovered over, it will change the button's background color to `HOVERED_BUTTON_COLOR`.
/// 4. For any other interaction state or button type, it will set the button's background color to `NORMAL_BUTTON_COLOR`.
///
/// # Examples:
/// To ensure that button interactions within the pause menu are properly handled, add this system to the app's update loop:
///
/// ```rust
/// app.add_system(button_interaction.system());
/// ```
///
/// # Notes:
/// Ensure the required constants (`PRESSED_BUTTON_COLOR`, `HOVERED_BUTTON_COLOR`, and `NORMAL_BUTTON_COLOR`) are defined and imported.
pub fn button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &PauseMenuButton),
        Changed<Interaction>,
    >,
    mut next_game_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut background_color, pause_button_option) in button_query.iter_mut() {
        match (*interaction, pause_button_option) {
            (Interaction::Pressed, PauseMenuButton::Running) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_game_state.set(GameState::Running);
            }
            (Interaction::Pressed, PauseMenuButton::MainMenu) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_game_state.set(GameState::Inactive);
            }
            (Interaction::Hovered, _) => *background_color = HOVERED_BUTTON_COLOR.into(),
            _ => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

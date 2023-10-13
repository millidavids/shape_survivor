use bevy::{app::AppExit, prelude::*};

use crate::states::AppState;

use super::{
    components::{MainMenu, MainMenuButton},
    styles::{
        get_button_text, HOVERED_BUTTON_COLOR, MAIN_MENU_STYLE, NORMAL_BUTTON_COLOR,
        NORMAL_BUTTON_STYLE, PRESSED_BUTTON_COLOR,
    },
};

/// Spawns the main menu UI components in the game.
///
/// This system initializes and spawns the main menu UI components which include a main container (node) and two buttons: "Play" and "Quit".
///
/// # Components
///
/// 1. **Main Node**: This is the main container for the main menu. It uses the `MAIN_MENU_STYLE` for styling.
/// 2. **Play Button**: A button that players can click to start or continue the game. It uses the `NORMAL_BUTTON_STYLE` for styling and displays the "Play" text.
/// 3. **Quit Button**: A button that players can click to quit or exit the game. It uses the `NORMAL_BUTTON_STYLE` for styling and displays the "Quit" text.
///
/// # Parameters
///
/// - `commands`: A mutable reference to the Bevy command buffer. This allows you to queue up changes to the world.
/// - `asset_server`: A reference to the Bevy asset server, used to load and manage assets such as fonts, textures, etc. In this context, it's used to fetch the button texts.
///
/// # Subsystems
///
/// The `get_button_text` function is used internally to retrieve the text for the buttons from the asset server.
///
/// # Example
///
/// This system can be added to your app's update loop, typically in the setup phase or when transitioning to the main menu scene:
///
/// ```rust
/// App::new()
///     .add_system(OnEnter, spawn_main_menu)
///     // other app configurations...
///     .run();
/// ```
///
/// Note: Ensure that `MAIN_MENU_STYLE` and `NORMAL_BUTTON_STYLE` are defined and accessible from where this system is defined.
///
pub fn spawn_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
        .spawn((
            NodeBundle {
                style: MAIN_MENU_STYLE,
                ..default()
            },
            MainMenu {},
        ))
        .with_children(|parent| {
            // ---- Play Button ----
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        ..default()
                    },
                    MainMenuButton::Play,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, "Play"),
                        ..default()
                    });
                });
            // ---- Quit Button ----
            parent
                .spawn((
                    ButtonBundle {
                        style: NORMAL_BUTTON_STYLE,
                        ..default()
                    },
                    MainMenuButton::Quit,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: get_button_text(&asset_server, "Quit"),
                        ..default()
                    });
                });
        });
}

/// Despawns the main menu and its associated UI components from the game world.
///
/// This system identifies the main menu entity by the `MainMenu` component and recursively despawns it,
/// ensuring that both the main menu and all its child entities (like buttons or other UI elements) are removed.
///
/// # Parameters
///
/// - `commands`: A mutable reference to the Bevy command buffer. This provides the ability to queue changes to the world,
///   including despawning entities.
/// - `main_menu_query`: A query designed to fetch entities tagged with the `MainMenu` component.
///
/// # Usage
///
/// Typically, you would use this system when transitioning away from the main menu, such as when a player decides to start the game
/// or when the main menu should be hidden for any other reason.
///
/// ```rust
/// App::new()
///     .add_system(OnEnter, despawn_main_menu)
///     // other app configurations...
///     .run();
/// ```
///
/// Note: Ensure that the `MainMenu` component/tag is defined and accessible from the location where this system is used.
///
pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

/// Handles interactions with main menu buttons and applies visual feedback.
///
/// This system responds to button interactions by changing the background color of the button based on the current interaction state
/// (e.g., pressed, hovered). Additionally, for specific button interactions, it updates the application's state or triggers app exit events.
///
/// - When the "Play" button is pressed, the game's next state is set to `AppState::Game`.
/// - When the "Quit" button is pressed, an `AppExit` event is sent to initiate the shutdown of the application.
///
/// # Parameters
///
/// - `button_query`: A query to fetch entities that have changed their interaction state and are tagged with `MainMenuButton`.
///   This query also gets the current `Interaction` state and the mutable `BackgroundColor` for each button.
/// - `next_app_state`: A mutable resource to set the next application state. Used for transitioning between game states.
/// - `app_exit_event_writer`: An event writer to send `AppExit` events, which can initiate the application shutdown process.
///
/// # Usage
///
/// Integrate this system in the main app builder to handle main menu button interactions:
///
/// ```rust
/// App::new()
///     .add_system(Update, button_interaction)
///     // other app configurations...
///     .run();
/// ```
///
/// Note: Make sure that the constants `PRESSED_BUTTON_COLOR`, `HOVERED_BUTTON_COLOR`, and `NORMAL_BUTTON_COLOR`
/// are defined and accessible. Similarly, ensure that `MainMenuButton` and `AppState` enumerations are available.
///
pub fn button_interaction(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor, &MainMenuButton),
        Changed<Interaction>,
    >,
    mut next_app_state: ResMut<NextState<AppState>>,
    mut app_exit_event_writer: EventWriter<AppExit>,
) {
    for (interaction, mut background_color, menu_button_option) in button_query.iter_mut() {
        match (*interaction, menu_button_option) {
            (Interaction::Pressed, MainMenuButton::Play) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                next_app_state.set(AppState::Game);
            }
            (Interaction::Pressed, MainMenuButton::Quit) => {
                *background_color = PRESSED_BUTTON_COLOR.into();
                app_exit_event_writer.send(AppExit);
            }
            (Interaction::Hovered, _) => *background_color = HOVERED_BUTTON_COLOR.into(),
            _ => *background_color = NORMAL_BUTTON_COLOR.into(),
        }
    }
}

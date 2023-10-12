use bevy::prelude::Component;

/// Represents the pause menu entity in the game.
///
/// When attached to an entity, this component signifies that the entity is a part of the pause menu.
/// This component can be queried to perform operations specific to the pause menu, such as spawning or despawning
/// the pause menu UI.
#[derive(Component)]
pub struct PauseMenu;

/// Represents the different buttons available in the pause menu.
///
/// This component is used to differentiate between the various buttons in the pause menu, allowing
/// for specific behavior to be associated with each button type.
///
/// # Variants:
/// - `Running`: Represents the button to resume or continue the game.
/// - `MainMenu`: Represents the button to return to the main menu.
#[derive(Component)]
pub enum PauseMenuButton {
    Running,
    MainMenu,
}
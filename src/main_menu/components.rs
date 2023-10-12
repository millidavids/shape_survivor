use bevy::prelude::Component;

/// Marker component for the main menu's root entity.
///
/// The `MainMenu` component is used to tag the main entity representing the game's main menu.
/// This marker allows systems to easily identify, query, and manipulate the main menu's root entity.
/// It can be particularly useful for tasks like spawning, updating, or despawning the entire main menu.
///
/// As it's a marker component without any data, its primary purpose is for organizational and querying convenience 
/// within the game's ECS architecture.
///
/// # Usage
///
/// You can attach this component to entities during their creation to specify their association with the main menu:
///
/// ```rust
/// commands.spawn(SomeBundle {
///     // ... other configurations ...
/// }).insert(MainMenu);
/// ```
///
/// Systems can then query entities with this component to handle specific main menu-related operations.
///
#[derive(Component)]
pub struct MainMenu;

/// Represents the different types of buttons in the main menu.
///
/// The `MainMenuButton` component is used to tag and differentiate between various buttons 
/// present within the main menu of the game. By associating this component with entities, 
/// systems can easily identify and apply specific logic based on the type of button.
///
/// Currently, there are two kinds of buttons:
/// 
/// - `Play`: Represents the button used to start or resume the game.
/// - `Quit`: Represents the button used to exit the game.
///
/// # Usage
///
/// You can attach this component to entities during their creation to specify their role in the main menu:
///
/// ```rust
/// commands.spawn(ButtonBundle {
///     // ... other configurations ...
/// }).insert(MainMenuButton::Play);
/// ```
///
/// Systems can then query entities with this component to handle button-specific logic, 
/// such as transitioning between game states or triggering other in-game actions.
///
#[derive(Component)]
pub enum MainMenuButton {
    Play,
    Quit,
}

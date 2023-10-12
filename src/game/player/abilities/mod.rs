mod components;
mod dot;

use bevy::prelude::*;

use self::dot::DotPlugin;

pub const DEFAULT_ABILITY_SPEED: f32 = 500.0;

/// A Bevy plugin for managing character abilities in a game.
///
/// The `AbilitiesPlugin` is responsible for adding and configuring the necessary Bevy plugins and systems
/// related to character abilities. It is typically used to integrate abilities, such as skills or powers,
/// into a Bevy game.
///
/// # Example
///
/// ```rust
/// use bevy::prelude::*;
///
/// struct AbilitiesPlugin;
///
/// impl Plugin for AbilitiesPlugin {
///     fn build(&self, app: &mut App) {
///         app.add_plugins(AbilitiesPlugin); // Add the AbilitiesPlugin to the Bevy app.
///         // ... Add other systems and setup for character abilities.
///     }
/// }
///
/// fn main() {
///     App::build()
///         .add_plugin(AbilitiesPlugin) // Add the AbilitiesPlugin to your Bevy app.
///         .run();
/// }
/// ```
///
/// In the example above, the `AbilitiesPlugin` is added to the Bevy app, allowing you to incorporate character abilities
/// into your game. You can use this plugin in conjunction with other systems and components to create a fully functional
/// abilities system for your characters.
///
/// # See Also
///
/// - [`Plugin`](https://bevyengine.github.io/bevy/bevy/app/trait.Plugin.html): Bevy trait for defining plugins.
/// - [`spawn_characters`](fn.spawn_characters.html): Bevy system for spawning character entities with abilities.
/// - [`Character`](struct.Character.html): A Bevy component that represents a character entity.
pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DotPlugin);
    }
}
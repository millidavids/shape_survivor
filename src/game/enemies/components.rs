use bevy::prelude::*;
use rand::Rng;

#[derive(Component)]
pub struct Enemy {
    pub targetable: bool,
}

#[derive(Component)]
pub struct HordeMover {
    /// The movement direction of the horde entity.
    pub dxdy: Vec3,
}

impl Default for HordeMover {
    /// Creates a new `HordeMover` component with a zeroed movement direction.
    fn default() -> Self {
        HordeMover { dxdy: Vec3::ZERO }
    }
}

impl HordeMover {
    /// Introduces random noise to the movement direction of the horde entity.
    ///
    /// The `noise` method is used to apply random noise to the existing movement direction of the horde entity.
    /// It modifies the `dxdy` field, which represents the movement direction in three dimensions (X, Y, and Z).
    /// By calling this method, you can add variability to the horde's movement pattern, making it appear less predictable.
    ///
    /// This can be particularly useful for creating natural-looking or chaotic movement behaviors for horde entities.
    /// The noise introduced is random and falls within a range of -100.0 to 100.0 along both the X and Y axes.
    ///
    /// # Example
    ///
    /// To create a `HordeMover` component for a horde entity with random noise applied to its movement direction:
    ///
    /// ```rust
    /// use bevy::prelude::*;
    ///
    /// struct HordePlugin;
    ///
    /// impl Plugin for HordePlugin {
    ///     fn build(&self, app: &mut App) {
    ///         app.add_systems(StartUp, setup_horde_entity);
    ///     }
    /// }
    ///
    /// fn setup_horde_entity(mut commands: Commands) {
    ///     let mut mover = HordeMover::default();
    ///     mover.noise(); // Introduce random noise to the movement direction
    ///
    ///     commands.spawn().insert(mover);
    /// }
    /// ```
    ///
    /// In the example above, the `noise` method is called on a `HordeMover` component to add randomness to the horde entity's movement direction during setup.
    ///
    /// # Notes
    ///
    /// The range of -100.0 to 100.0 for noise values is arbitrary and can be adjusted to suit the desired level of randomness in the movement.
    ///
    /// It's recommended to call the `noise` method when initially setting up or updating the horde entity's movement behavior.
    /// Repeated calls to this method will alter the movement direction each time it is invoked.
    ///
    /// Be cautious when using high noise values, as they can result in erratic and unpredictable movement patterns.
    /// Adjust the noise range as needed to achieve the desired balance between randomness and controlled movement.
    ///
    pub fn noise(&mut self) {
        self.dxdy.x = rand::thread_rng().gen_range(-100.0..100.0);
        self.dxdy.y = rand::thread_rng().gen_range(-100.0..100.0);
    }
}

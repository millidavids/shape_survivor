use bevy::prelude::*;

/// Represents a range of indices for a sprite sheet animation sequence.
///
/// This component is used to determine the frame range for an animation and keep track of the animation direction.
/// It supports both forward and reverse looping animations. For example, given an `AnimationIndices` with
/// `first` as 2, `last` as 5, the animation sequence will be 2 -> 3 -> 4 -> 5 -> 4 -> 3 -> 2 -> 3 -> ...
///
/// # Attributes
/// - `first`: The index of the first frame in the animation sequence.
/// - `last`: The index of the last frame in the animation sequence.
/// - `reverse`: A flag to determine if the animation is currently playing in reverse.
#[derive(Component, Clone, Copy)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub reverse: bool,
}

impl AnimationIndices {
    /// Updates the current animation index based on the provided index and internal state.
    ///
    /// This function will loop the animation forward until it reaches the `last` index and then loop
    /// it backward until it reaches the `first` index. The function updates the `reverse` flag as needed.
    ///
    /// # Parameters
    /// - `index`: A reference to the current animation index.
    ///
    /// # Returns
    /// - A new animation index based on the provided `index` and the internal state.
    pub fn tick(&mut self, index: &usize) -> usize {
        if !self.reverse {
            if *index == self.last {
                self.reverse = !self.reverse;
                *index - 1
            } else {
                *index + 1
            }
        } else {
            if *index == self.first {
                self.reverse = !self.reverse;
                *index + 1
            } else {
                *index - 1
            }
        }
    }
}

/// A timer specifically designed for sprite sheet animations.
///
/// This component wraps around the core `Timer` struct, giving it a more semantically meaningful name
/// for use in animation contexts. This timer can be used to control the timing and speed of sprite animations.
///
/// Using `Deref` and `DerefMut` traits, you can directly access the inner `Timer`'s methods and fields.
///
/// # Usage
/// ```rust
/// let mut animation_timer = AnimationTimer(Timer::from_seconds(0.1, true));
/// if animation_timer.0.finished(&time) { 
///     // Handle animation frame update 
/// }
/// ```
#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);
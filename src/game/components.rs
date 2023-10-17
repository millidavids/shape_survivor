use bevy::prelude::*;

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

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct Health(pub f32);

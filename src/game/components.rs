use bevy::prelude::*;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
    pub reverse: bool,
}

impl AnimationIndices {
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
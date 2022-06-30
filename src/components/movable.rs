use bevy::prelude::{ Component, Vec2 };

#[derive(Component, Copy, Clone, Debug, Default)]
pub struct Movable {
    pub motion: Vec2,
}

impl Movable {
    pub fn new() -> Self {
        Self {
            motion: Vec2::ZERO
        }
    }

    pub fn with_motion(mut self, value: Vec2) -> Self {
        self.motion = value;
        self
    }
}
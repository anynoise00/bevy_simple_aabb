pub mod rectangle;

use bevy::prelude::{ Component, Vec2 };
pub use rectangle::*;

#[derive(Component, Default)]
pub struct KinematicBody {
    pub shape: Rectangle,
    pub motion: Vec2,
}

#[derive(Component, Default)]
pub struct StaticBody {
    pub shape: Rectangle,
}
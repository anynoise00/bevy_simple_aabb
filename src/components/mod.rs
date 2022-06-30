pub mod rectangle;
pub mod movable;

use bevy::prelude::{ Bundle, Component };
pub use movable::*;
pub use rectangle::*;

#[derive(Component, Default)]
pub struct Body;

#[derive(Bundle, Default)]
pub struct MovableBundle {
    pub body: Body,
    pub movable: Movable,
    pub rectangle: Rectangle,
}

#[derive(Bundle, Default)]
pub struct StaticBundle {
    pub body: Body,
    pub rectangle: Rectangle,
}
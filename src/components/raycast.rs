use bevy::{prelude::{ Bundle, Component, Entity, Vec2 }, transform::TransformBundle};
use crate::collision::ray::Hit;

#[derive(Bundle, Default)]
pub struct RaycastBundle {
    pub raycast: Raycast,

    #[bundle]
    pub transform_bundle: TransformBundle,
}

#[derive(Component, Default)]
pub struct Raycast {
    pub direction: Vec2,
    pub offset: Vec2,

    pub(crate) hits: Vec<(Entity, Hit)>,
}

impl Raycast {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_direction(mut self, direction: Vec2) -> Self {
        self.direction = direction;
        self
    }

    pub fn with_offset(mut self, offset: Vec2) -> Self {
        self.offset = offset;
        self
    }

    pub fn get_hits(&self) -> Vec<(Entity, Hit)> {
        self.hits.clone()
    }

    pub fn is_colliding(&self) -> bool {
        self.hits.len() > 0
    }
}

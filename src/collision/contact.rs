use bevy::prelude::{ Entity, Vec2 };

#[derive(Clone, Copy, Debug)]
pub struct Contact {
    pub(crate) entity: Entity,
    pub(crate) normal: Vec2,
}

impl Contact {
    pub fn entity(&self) -> Entity {
        self.entity
    }

    pub fn normal(&self) -> Vec2 {
        self.normal
    }
}
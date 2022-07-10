pub mod raycast;
pub mod rectangle;

use bevy::prelude::{ Component, Vec2 };
use crate::collision::Contact;
pub use raycast::*;
pub use rectangle::*;

#[derive(Component, Default)]
pub struct KinematicBody {
    pub shape: Rectangle,
    pub motion: Vec2,

    pub(crate) contacts: Vec<Contact>,
}

impl KinematicBody {
    pub fn new(shape: Rectangle) -> Self {
        Self {
            shape,
            motion: Vec2::ZERO,

            contacts: Vec::new(),
        }
    }

    pub fn get_contacts(&self) -> Vec<Contact> {
        self.contacts.clone()
    }
}

#[derive(Component, Default)]
pub struct StaticBody {
    pub shape: Rectangle,
}

impl StaticBody {
    pub fn new(shape: Rectangle) -> Self {
        Self {
            shape,
        }
    }
}
pub mod components;
pub mod collision;
pub mod plugin;
pub mod utils;

pub mod prelude {
    pub use crate::components::{ KinematicBody, StaticBody, Rectangle };
    pub use crate::collision::{ Raycast, RaycastBundle };
    pub use crate::plugin::PhysicsPlugin;
}
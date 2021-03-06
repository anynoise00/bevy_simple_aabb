pub mod components;
pub mod collision;
pub mod plugin;
pub mod utils;

pub mod prelude {
    pub use crate::components::{ KinematicBody, StaticBody, Raycast, RaycastBundle, Rectangle };
    pub use crate::plugin::PhysicsPlugin;
}
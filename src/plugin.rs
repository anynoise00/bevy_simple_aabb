use bevy::prelude::*;
use crate::collision::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<MoveEvent>()
            .add_system(narrowphase)
            .add_system(move_entities.after(narrowphase))
            .add_system(debug_positions.after(move_entities));
    }
}
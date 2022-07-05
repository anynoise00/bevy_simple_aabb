use bevy::prelude::*;
use crate::collision::*;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BroadEvent>()
            .add_event::<NarrowEvent>()
            .add_event::<MoveEvent>()
            
            .add_system(broadphase)
            .add_system(narrowphase.after(broadphase))
            .add_system(solve.after(narrowphase))
            .add_system(move_entities.after(solve));
    }
}
use bevy::prelude::*;
use crate::collision::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, StageLabel)]
pub struct PhysicsStage;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<BroadEvent>()
            .add_event::<NarrowEvent>()
            .add_event::<MoveEvent>()
            
            .add_stage_after(CoreStage::Update, PhysicsStage, SystemStage::single_threaded())
            .add_system_to_stage(PhysicsStage, broadphase)
            .add_system_to_stage(PhysicsStage, narrowphase.after(broadphase))
            .add_system_to_stage(PhysicsStage, clear_contacts.before(solve))
            .add_system_to_stage(PhysicsStage, solve.after(narrowphase))
            .add_system_to_stage(PhysicsStage, move_entities.after(solve))
            .add_system_to_stage(PhysicsStage, raycasts.after(move_entities));
    }
}
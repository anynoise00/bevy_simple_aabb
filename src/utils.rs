use bevy::prelude::Vec2;

pub const EPSILON: f32 = 0.0000001;

pub fn slide_motion(motion: &mut Vec2, normal: Vec2, time: f32) {
    *motion += motion.abs() * normal * (1.0 - time - EPSILON)
}
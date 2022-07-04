use bevy::{prelude::*, utils::{HashMap, HashSet}};
use crate::{components::*, aabb::Aabb};

pub struct MoveEvent {
    entity: Entity,
    position: Vec2,
}

pub struct BroadEvent {
    entity: Entity,

    possible: PossibleCollisions,
}

struct PossibleCollisions {
    kin: Entity,
    sta: Entity,
}

pub fn broadphase(
    kinematics: Query<(Entity, &KinematicBody)>,
    statics: Query<(Entity, &StaticBody)>,

    mut ev_broad: EventWriter<BroadEvent>,
    mut ev_move: EventWriter<MoveEvent>,
) {

}

pub fn narrowphase(
    kinematics: Query<(Entity, &KinematicBody, &Transform)>,
    statics: Query<(Entity, &StaticBody, &Transform)>,

    mut ev_move: EventWriter<MoveEvent>,
) {
    for (a_ent, a_body, a_trans) in kinematics.iter() {
        let mut a_box = Aabb::new(a_body.shape, a_trans);
        a_box.position += a_body.motion;

        for (_, b_body, b_trans) in statics.iter() {
            let b_box = Aabb::new(b_body.shape, b_trans);

            if a_box.is_overlapping(b_box) {
                let overlap = a_box.get_overlap(b_box);
                a_box.position += overlap;
                println!("{} c", a_ent.id());
            }
        }

        ev_move.send(MoveEvent {
            entity: a_ent,
            position: a_box.position,
        });
    }
}

pub fn move_entities(
    mut q: Query<&mut Transform, With<KinematicBody>>,
    mut ev_move: EventReader<MoveEvent>,
) {
    for ev in ev_move.iter() {
        let mut transform = match q.get_component_mut::<Transform>(ev.entity) {
            Ok(t) => t,
            Err(e) => {
                println!("Entity {} error. {}", ev.entity.id(), e);
                continue;
            },
        };

        transform.translation.x = ev.position.x;
        transform.translation.y = ev.position.y;
    }
}
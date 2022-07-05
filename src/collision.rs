use bevy::prelude::*;
use crate::{components::*, aabb::Aabb};

pub struct BroadEvent {
    pub entity: Entity,

    pub kinematics: Vec<Entity>,
    pub statics: Vec<Entity>,
}

pub struct NarrowEvent {
    pub entity: Entity,

    pub kinematics: Vec<(Entity, f32)>,
    pub statics: Vec<(Entity, f32)>,
}

pub struct MoveEvent {
    pub entity: Entity,
    pub position: Vec2,
}

pub fn broadphase(
    kinematics: Query<(Entity, &KinematicBody, &Transform)>,
    statics: Query<(Entity, &StaticBody, &Transform)>,

    mut ev_broad: EventWriter<BroadEvent>,
) {
    for (a_ent, a_body, a_trans) in kinematics.iter() {
        let mut kin = Vec::<Entity>::new();
        let mut sta = Vec::<Entity>::new();
        let a_box = Aabb::new(a_body.shape, a_trans).get_broad(a_body.motion);

        for (b_ent, b_body, b_trans) in statics.iter() {
            let b_box = Aabb::new(b_body.shape, b_trans);

            if a_box.is_overlapping(b_box) {
                sta.push(b_ent);
            }
        }

        ev_broad.send(BroadEvent {
            entity: a_ent,
            kinematics: kin,
            statics: sta,
        });
    }
}

pub fn narrowphase(
    kinematics: Query<(&KinematicBody, &Transform)>,
    statics: Query<(&StaticBody, &Transform)>,

    mut ev_broad: EventReader<BroadEvent>,
    mut ev_narrow: EventWriter<NarrowEvent>,
) {
    for ev in ev_broad.iter() {
        let (a_body, a_trans) = match kinematics.get(ev.entity) {
            Ok((body, trans)) => (body, trans),
            Err(_) => continue,
        };
        let a_box = Aabb::new(a_body.shape, a_trans);

        let mut sta_col: Vec<(Entity, f32,)> = Vec::new();
        for b_ent in ev.statics.iter() {
            let b_ent = *b_ent;

            let (b_body, b_trans) = match statics.get(b_ent) {
                Ok((body, trans)) => (body, trans),
                Err(_) => continue,
            };
            let b_box = Aabb::new(b_body.shape, b_trans);

            let (time, _) = a_box.get_hit_info(b_box, a_body.motion);
            if time < 1.0 {
                sta_col.push((b_ent, time));
            }
        }

        println!("{:?}", sta_col);
        sta_col.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        println!("{:?}", sta_col);
        println!();

        ev_narrow.send(NarrowEvent {
            entity: ev.entity,
            kinematics: Vec::new(),
            statics: sta_col,
        })
    }
}

pub fn solve(
    mut kinematics: Query<(&mut KinematicBody, &Transform)>,
    statics: Query<(&StaticBody, &Transform)>,

    mut ev_narrow: EventReader<BroadEvent>,
    mut ev_move: EventWriter<MoveEvent>,
) {
    for ev in ev_narrow.iter() {
        let (a_body, a_trans) = match kinematics.get_mut(ev.entity) {
            Ok((body, trans)) => (body, trans),
            Err(_) => continue,
        };
        let mut a_box = Aabb::new(a_body.shape, a_trans);
        a_box.position += a_body.motion;

        for b_ent in ev.statics.iter() {
            let b_ent = *b_ent;

            let (b_body, b_trans) = match statics.get(b_ent) {
                Ok((body, trans)) => (body, trans),
                Err(_) => continue,
            };
            let b_box = Aabb::new(b_body.shape, b_trans);

            if a_box.is_overlapping(b_box) {
                let overlap = a_box.get_overlap(b_box);
                a_box.position += overlap;
            }
        }

        ev_move.send(MoveEvent {
            entity: ev.entity,
            position: a_box.position,
        })
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
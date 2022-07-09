pub mod aabb;
pub mod raycast;

use bevy::prelude::*;
use crate::components::*;
use self::utils::slide_motion;

pub use aabb::*;
pub use raycast::*;

const DIAGONAL_SOLVE: Vec2 = Vec2::X;

pub struct BroadEvent {
    pub entity: Entity,

    pub kinematics: Vec<Entity>,
    pub statics: Vec<Entity>,
}

pub struct NarrowEvent {
    pub entity: Entity,

    pub kinematics: Vec<Collisions>,
    pub statics: Vec<Collisions>,
}

pub struct MoveEvent {
    pub entity: Entity,
    pub position: Vec2,
}

pub struct Collisions {
    pub time: f32,
    pub entities: Vec<Entity>,
}

pub fn broadphase(
    kinematics: Query<(Entity, &KinematicBody, &Transform)>,
    statics: Query<(Entity, &StaticBody, &Transform)>,

    mut ev_broad: EventWriter<BroadEvent>,
) {
    for (a_ent, a_body, a_trans) in kinematics.iter() {
        let kin = Vec::<Entity>::new();
        let mut sta = Vec::<Entity>::new();
        let a_box = Aabb::from_rectangle(a_body.shape, a_trans).get_broad(a_body.motion);

        for (b_ent, b_body, b_trans) in statics.iter() {
            let b_box = Aabb::from_rectangle(b_body.shape, b_trans);

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
        let a_box = Aabb::from_rectangle(a_body.shape, a_trans);

        let mut sta_col: Vec<Collisions> = Vec::new();
        for b_ent in ev.statics.iter() {
            let b_ent = *b_ent;

            let (b_body, b_trans) = match statics.get(b_ent) {
                Ok((body, trans)) => (body, trans),
                Err(_) => continue,
            };
            let b_box = Aabb::from_rectangle(b_body.shape, b_trans);

            match a_box.sweep_test(b_box, a_body.motion) {
                Some(hit) => {
                    match sta_col.iter_mut().find(|col| col.time == hit.time) {
                        Some(col) => col.entities.push(b_ent),
                        None => sta_col.push(Collisions { time: hit.time, entities: vec![b_ent] }),
                    }
                    
                },
                None => continue,
            };
        }

        sta_col.sort_by(|a, b| (a.time).partial_cmp(&b.time).unwrap());

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

    mut ev_narrow: EventReader<NarrowEvent>,
    mut ev_move: EventWriter<MoveEvent>,
) {
    for ev in ev_narrow.iter() {
        let (a_body, a_trans) = match kinematics.get_mut(ev.entity) {
            Ok((body, trans)) => (body, trans),
            Err(_) => continue,
        };
        let a_box = Aabb::from_rectangle(a_body.shape, a_trans);
        let mut a_motion = a_body.motion;

        for col in ev.statics.iter() {
            for b_ent in col.entities.iter() {
                let (b_body, b_trans) = match statics.get(*b_ent) {
                    Ok((body, trans)) => (body, trans),
                    Err(_) => continue,
                };
                let mut b_box = Aabb::from_rectangle(b_body.shape, b_trans);
                let mut is_diagonal = false;
    
                if !a_box.get_broad(a_motion).is_overlapping(b_box) { continue; }
                match a_box.sweep_test(b_box, a_motion) {
                    Some(hit) => {
                        if hit.normal == Vec2::ZERO && col.entities.len() <= 1 {
                            is_diagonal = true;
                        }
                        slide_motion(&mut a_motion, hit.normal, hit.time);
                    },
                    None => continue,
                }

                if is_diagonal {
                    b_box.extents += a_box.extents * DIAGONAL_SOLVE;

                    match a_box.sweep_test(b_box, a_motion) {
                        Some(hit) => slide_motion(&mut a_motion, hit.normal, hit.time),
                        None => continue,
                    }
                }
            }
        }

        ev_move.send(MoveEvent {
            entity: ev.entity,
            position: a_box.position + a_motion,
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

pub fn raycasts(
    mut rays: Query<(&mut Ray, &Transform)>,
    statics: Query<(Entity, &StaticBody, &Transform)>,
) {
    for (mut a_ray, a_trans) in rays.iter_mut() {
        let a_box = Aabb::from_ray(&a_ray, a_trans);
        let raycast = Raycast::from_ray(&a_ray, a_trans);
        a_ray.hits.clear();

        for (b_ent, b_body, b_trans) in statics.iter() {
            let b_box = Aabb::from_rectangle(b_body.shape, b_trans);

            if a_box.is_overlapping(b_box) {
                match raycast.intersect_aabb(b_box) {
                    Some(hit) => a_ray.hits.push((b_ent, hit)),
                    None => continue,
                }
            }
        }

    }
}

pub mod utils {
    use bevy::prelude::Vec2;

    pub const EPSILON: f32 = 0.0000001;

    pub fn slide_motion(motion: &mut Vec2, normal: Vec2, time: f32) {
        *motion += motion.abs() * normal * (1.0 - time - EPSILON)
    }
}
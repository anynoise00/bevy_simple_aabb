use bevy::prelude::*;
use crate::{components::*, aabb::Aabb};

pub struct MoveEvent {
    entity: Entity,
    position: Vec2,
}

pub fn narrowphase(
    movables: Query<(Entity, &Rectangle, &Movable, &Transform), With<Body>>,
    statics: Query<(Entity, &Rectangle, &Transform), (With<Body>, Without<Movable>)>,

    mut ev_move: EventWriter<MoveEvent>,
) {
    for (a_ent, a_rect, a_mov, a_trans) in movables.iter() {
        let mut a_box = Aabb::new(a_rect, a_trans);
        a_box.position += a_mov.motion;

        for (_, b_rect, b_trans) in statics.iter() {
            let b_box = Aabb::new(b_rect, b_trans);

            if a_box.collide_with(b_box) {
                let overlap = a_box.get_overlap(b_box);
                a_box.position += overlap;
            }
        }

        ev_move.send(MoveEvent {
            entity: a_ent,
            position: a_box.position,
        });
    }
}

pub fn move_entities(
    mut q: Query<&mut Transform, With<Movable>>,
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

pub fn debug_positions(
    movables: Query<(Entity, &Transform), (With<Body>, With<Movable>)>,
    statics: Query<(Entity, &Transform), (With<Body>, Without<Movable>)>, 
) {
    for (ent, trans) in movables.iter() {
        println!("MOVABLE entity {} is at position {}", ent.id(), trans.translation);
    }
    
    for (ent, trans) in statics.iter() {
        println!("STATIC entity {} is at position {}", ent.id(), trans.translation);
    }

    println!();
}
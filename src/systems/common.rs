use crate::{
    collide::{collide, update_vel},
    components::*,
};
use specs::prelude::*;
use std::collections::HashMap;

pub struct UpdateVel;

impl<'a> System<'a> for UpdateVel {
    type SystemData = (WriteStorage<'a, Vel>, ReadStorage<'a, Acc>);

    fn run(&mut self, (mut vel, acc): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            *vel += *acc;
        }
    }
}

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (WriteStorage<'a, Pos>, WriteStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            *pos += *vel;
        }
    }
}

pub struct ReduceVel;

impl<'a> System<'a> for ReduceVel {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Lives>,
        ReadStorage<'a, Damage>,
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Class>,
        ReadStorage<'a, Block>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, pos, siz, mut vel, mut lives, dmg, bullet, mut ply, cls, blk, lazy): Self::SystemData,
    ) {
        let mut map = HashMap::<_, Vel>::new();

        // Player v.s. block
        for (e1, p1, s1, _) in (&e, &pos, &siz, &ply).join() {
            for (e2, p2, s2, _) in (&e, &pos, &siz, &blk).join() {
                let z = Vel::zero();
                let v1 = vel.get(e1).unwrap_or(&z);
                let v2 = vel.get(e2).unwrap_or(&z);

                let (v1, v2) = update_vel(p1, s1, v1, p2, s2, v2);

                if let Some(vel) = map.get_mut(&e1) {
                    *vel = vel.min(&v1);
                } else {
                    map.insert(e1, v1);
                }
                if let Some(vel) = map.get_mut(&e2) {
                    *vel = vel.min(&v2);
                } else {
                    map.insert(e2, v2);
                }
            }
        }

        for (e, v) in map {
            match vel.get_mut(e) {
                Some(vel) => {
                    *vel = v;
                }
                None => {}
            }
        }
    }
}

pub struct CheckCollide;

impl<'a> System<'a> for CheckCollide {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Lives>,
        ReadStorage<'a, Damage>,
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Class>,
        ReadStorage<'a, Block>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, pos, siz, mut vel, mut lives, dmg, bullet, mut ply, cls, blk, lazy): Self::SystemData,
    ) {
        // Player v.s. bullet
        for (e1, p1, s1, cls1, _) in (&e, &pos, &siz, &cls, &bullet).join() {
            for (e2, p2, s2, cls2, _) in (&e, &pos, &siz, &cls, &ply).join() {
                if cls1 == cls2 {
                    continue;
                }
                let z = Vel::zero();
                let v1 = vel.get(e1).unwrap_or(&z);
                let v2 = vel.get(e2).unwrap_or(&z);

                if collide(p1, s1, v1, p2, s2, v2) {
                    let dmg = dmg.get(e1).map(|dmg| dmg.0).unwrap_or(0);

                    // Delete bullet
                    e.delete(e1);

                    // Subtract lives
                    if let Some(lives) = lives.get_mut(e2) {
                        lives.0 = lives.0.saturating_sub(dmg);

                        if lives.0 == 0 {
                            // Delete player too
                            e.delete(e2);
                        }
                    }
                }
            }
        }
    }
}

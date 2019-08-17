use crate::{
    collide::{collide, update_vel},
    components::*,
};
use log::*;
use specs::prelude::*;
use std::collections::HashMap;

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (WriteStorage<'a, Pos>, ReadStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            *pos += *vel;
        }
    }
}

pub struct UpdateVel;

impl<'a> System<'a> for UpdateVel {
    type SystemData = (
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, (mut vel, acc, mut ply): Self::SystemData) {
        for (vel, acc, ply) in (&mut vel, &acc, ply.maybe()).join() {
            if !ply.map(|p| p.land).unwrap_or(false) {
                *vel += *acc;
            } else {
                vel.x *= 0.98;
            }
        }
    }
}

pub struct ReduceVel;

impl<'a> System<'a> for ReduceVel {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Block>,
    );

    fn run(&mut self, (e, pos, mut vel, acc, siz, mut ply, blk): Self::SystemData) {
        let mut map = HashMap::<_, Vel>::new();

        // Player v.s. block
        for (e1, p1, s1, ply) in (&e, &pos, &siz, &mut ply).join() {
            let mut land = false;

            for (e2, p2, s2, _) in (&e, &pos, &siz, &blk).join() {
                let z = Vel::zero();
                let v1 = vel.get(e1).unwrap_or(&z);
                let v2 = vel.get(e2).unwrap_or(&z);

                let ((n, v1), (_, v2)) = update_vel(p1, s1, &v1, p2, s2, v2);

                if let Some(n) = n {
                    if n.y < 0.0 {
                        land = true;
                    }
                }

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

            ply.land = land;
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
        ReadStorage<'a, Vel>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, Player>,
    );

    fn run(&mut self, (e, pos, vel, siz, blt, mut ply): Self::SystemData) {
        // Player v.s. bullet
        for (e1, p1, s1, blt) in (&e, &pos, &siz, &blt).join() {
            for (e2, p2, s2, ply) in (&e, &pos, &siz, &mut ply).join() {
                if blt.cls == ply.cls {
                    continue;
                }
                let z = Vel::zero();
                let v1 = vel.get(e1).unwrap_or(&z);
                let v2 = vel.get(e2).unwrap_or(&z);

                if collide(p1, s1, v1, p2, s2, v2) {
                    // Delete bullet
                    let _ = e.delete(e1);

                    ply.lives = ply.lives.saturating_sub(blt.dmg);

                    if ply.lives == 0 {
                        // Player dies
                        let _ = e.delete(e2);
                    }
                }
            }
        }
    }
}

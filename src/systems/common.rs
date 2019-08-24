use crate::{
    collide::{cease_vel, collide, update_vel},
    components::*,
};
use log::*;
use specs::prelude::*;
use std::collections::HashMap;

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (mut pos, vel, ply): Self::SystemData) {
        for (pos, vel, _) in (&mut pos, &vel, !&ply).join() {
            *pos += *vel;
        }
    }
}

pub struct UpdateVel;

impl<'a> System<'a> for UpdateVel {
    type SystemData = (WriteStorage<'a, Vel>, ReadStorage<'a, Acc>);

    fn run(&mut self, (mut vel, acc): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            *vel += *acc;
        }
    }
}

pub struct ReduceVel;

impl<'a> System<'a> for ReduceVel {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Block>,
    );

    fn run(&mut self, (e, mut pos, mut vel, acc, siz, mut ply, blk): Self::SystemData) {
        for (e1, p1, s1, v1, ply) in (&e, &pos, &siz, &mut vel, &mut ply).join() {
            let mut land = false;

            for (e2, p2, s2, _) in (&e, &pos, &siz, &blk).join() {
                let (n, v) = cease_vel(&p1, &s1, &v1, &p2, &s2);

                *v1 = v;

                if let Some(n) = n {
                    if n.y < 0.0 {
                        land = true;
                    }
                }
            }

            ply.land = land;
        }

        let mut newpos = HashMap::new();

        for (e1, p1, s1, v1, ply) in (&e, &pos, &siz, &mut vel, &mut ply).join() {
            for (e2, p2, s2, _) in (&e, &pos, &siz, &blk).join() {
                let (v, _) = update_vel(p1, s1, v1, p2, s2, &Vel::zero());
                *v1 = v;
                newpos.insert(e1, *p1 + v);
            }
        }

        for (e, np) in newpos {
            let np = Pos::new(np.x.round(), np.y.round());
            if let Some(pos) = pos.get_mut(e) {
                *pos = np;
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

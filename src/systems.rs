use crate::{
    client::Client,
    collide::{cease_vel, collide, normal, toi, update_vel},
    components::*,
    config::Config,
    entities::*,
    error::Result,
    protocol::*,
    resources::*,
    vector::Vector,
};
use specs::{prelude::*, world::EntityBuilder};
use std::collections::HashMap;

use log::*;

struct Print;

impl<'a> System<'a> for Print {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>);

    fn run(&mut self, (e, pos): Self::SystemData) {
        for (e, pos) in (&e, &pos).join() {
            trace!("{:?}: pos={:?}", e, pos);
        }
    }
}

struct TakeAction;

impl<'a> System<'a> for TakeAction {
    type SystemData = (
        Entities<'a>,
        Read<'a, User>,
        Write<'a, Action>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Class>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Ori>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, user, mut act, player, cls, pos, siz, mut vel, acc, mut ori, lazy): Self::SystemData,
    ) {
        if !act.update {
            return;
        }

        let ep = match user.entity {
            Some(entity) => entity,
            None => return,
        };

        if act.jump {
            vel.get_mut(ep).map(|vel| {
                vel.y = 5.0;
            });
        }

        let pos = pos.get(ep).unwrap();
        let siz = siz.get(ep).unwrap();
        let player = player.get(ep).unwrap();
        let cls = cls.get(ep).unwrap();
        let mut vel = vel.get_mut(ep).unwrap();
        let mut ori = ori.get_mut(ep).unwrap();

        if act.right {
            vel.x = 5.0;
            ori.x = 1.0;
        }

        if act.left {
            vel.x = -5.0;
            ori.x = -1.0;
        }

        if act.take {
            let d = if ori.x > 0.0 { siz.x } else { -10.0 };
            lazy.create_entity(&e).create(BulletEntity::new(
                *pos + Vel::new(d, 0.0),
                Size::new(30.0, 30.0),
                Vel::new(10.0 * ori.x, 0.0),
                Acc::zero(),
                Asset(100),
                *ori,
                Owner(player.0),
                Class(cls.0),
                Damage(1),
            ));
        }

        act.clear();
    }
}

struct UpdateVel;

impl<'a> System<'a> for UpdateVel {
    type SystemData = (WriteStorage<'a, Vel>, ReadStorage<'a, Acc>);

    fn run(&mut self, (mut vel, acc): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            *vel += *acc;
        }
    }
}

struct UpdateCollide;

impl<'a> System<'a> for UpdateCollide {
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

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (WriteStorage<'a, Pos>, WriteStorage<'a, Vel>);

    fn run(&mut self, (mut pos, vel): Self::SystemData) {
        for (pos, vel) in (&mut pos, &vel).join() {
            *pos += *vel;
        }
    }
}

struct OutOfBound;

impl<'a> System<'a> for OutOfBound {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>, ReadStorage<'a, Player>);

    fn run(&mut self, (e, pos, ply): Self::SystemData) {
        // for (plypos, ply, _) in (&pos, &ply, &user).join() {
        //     for (e1, pos) in (&e, &pos).join() {
        //         let d = *pos - *plypos;
        //         if d.len() >= 2000.0 {
        //             e.delete(e1);
        //         }
        //     }
        // }
    }
}

pub struct Systems {
    pub(crate) world: World,
}

impl Systems {
    pub fn new() -> Result<Self> {
        let mut world = World::new();

        world.register::<Class>();
        world.register::<Player>();
        world.register::<Owner>();
        world.register::<Pos>();
        world.register::<Vel>();
        world.register::<Size>();
        world.register::<Acc>();
        world.register::<Ori>();
        world.register::<Lives>();
        world.register::<Bullet>();
        world.register::<Damage>();
        world.register::<Landmark>();
        world.register::<Background>();
        world.register::<Block>();
        world.register::<Asset>();
        world.insert(Action::default());
        world.insert(User::default());
        world.insert(Events::default());

        Ok(Self { world })
    }

    ///
    /// Execute one turn
    ///
    pub fn update(&mut self) {
        Print.run_now(&mut self.world);
        TakeAction.run_now(&mut self.world);
        UpdateVel.run_now(&mut self.world);
        UpdateCollide.run_now(&mut self.world);
        UpdatePos.run_now(&mut self.world);
        OutOfBound.run_now(&mut self.world);
        Print.run_now(&mut self.world);
        self.world.maintain();
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn world_mut(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn run<'a, T: System<'a>>(&'a mut self, mut sys: T) {
        sys.run_now(&mut self.world);
    }
}

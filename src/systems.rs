use crate::{
    client::Client, components::*, config::Config, error::Result, io::Io, protocol::*, resources::*,
};
use ncollide2d::{
    math::Isometry,
    query::{contact, time_of_impact},
    shape::Cuboid,
};
use specs::prelude::*;
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

struct Input<'a> {
    io: &'a mut Io,
}

impl<'a> Input<'a> {
    fn new(io: &'a mut Io) -> Self {
        Self { io }
    }
}

impl<'a> System<'a> for Input<'a> {
    type SystemData = (
        Entities<'a>,
        Write<'a, Action>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Dir>,
        Read<'a, LazyUpdate>,
        Read<'a, User>,
    );

    fn run(
        &mut self,
        (e, mut act, player, pos, siz, mut vel, acc, mut dir, lazy, user): Self::SystemData,
    ) {
        for (player, pos, siz, vel, acc, dir) in
            (&player, &pos, &siz, &mut vel, &acc, &mut dir).join()
        {
            if !user.is_me(player) {
                continue;
            }

            if !act.update {
                continue;
            }

            if act.jump {
                vel.y = 5.0;
            }
            if act.right {
                vel.x = 5.0;
                dir.0 = 1.0;
            }
            if act.left {
                vel.x = -5.0;
                dir.0 = -1.0;
            }
            if act.take {
                let d = if dir.0 > 0.0 { siz.x } else { -10.0 };
                lazy.create_entity(&e)
                    .with(Vel::new(10.0 * dir.0, 0.0))
                    .with(*pos + Vel::new(d, 0.0))
                    .with(Acc::zero())
                    .with(Bullet { class: CLASS_CHIBA })
                    .with(Size::new(30.0, 30.0))
                    .with(Asset(100))
                    .build();
            }

            let info = SendAction {
                player: player.clone(),
                pos: *pos,
                vel: *vel,
                acc: *acc,
                dir: *dir,
                action: act.clone(),
            };

            self.io.send_action(info).unwrap();
        }

        act.clear();
    }
}

struct BjarneSystem;

impl<'a> System<'a> for BjarneSystem {
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Pos>,
                       WriteStorage<'a, Boss>,
                       Read<'a, LazyUpdate>);

    fn run(&mut self, (e, pos, mut boss, lazy): Self::SystemData) {
        for (pos, boss) in (&pos, &mut boss).join() {
            boss.step = boss.step.wrapping_add(1);
            if boss.step % 100 == 0 {
                for i in (90..270).filter(|i| i % 10 == 0) {
                    use std::f32::consts::PI;
                    let x = (i as f32 / 180.0 * PI).cos() * 10.0;
                    let y = (i as f32 / 180.0 * PI).sin() * 10.0;
                    lazy.create_entity(&e)
                        .with(Vel::new(x, y))
                        .with(*pos + Pos::new(0.0, 200.0 * 0.8))
                        .with(Acc::zero())
                        .with(Size::new(30.0, 30.0))
                        .with(Asset(3))
                        .with(Player {
                            id: 1000,
                            cls: CLASS_SAITAMA,
                            lives: 10,
                        })
                        .with(GC)
                        .with(Enemy)
                        .with(Dir(-1.0))
                        .build();
                }
            }
        }
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

fn toi(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size, v2: &Vel) -> f32 {
    let m1 = *p1 + *s1 / 2.0;
    let m1 = Isometry::translation(m1.x, m1.y);
    let c1 = Cuboid::new((*s1 / 2.0).to_vec());
    let v1 = v1.to_vec();

    let m2 = *p2 + *s2 / 2.0;
    let m2 = Isometry::translation(m2.x, m2.y);
    let c2 = Cuboid::new((*s2 / 2.0).to_vec());
    let v2 = v2.to_vec();

    time_of_impact(&m1, &v1, &c1, &m2, &v2, &c2)
        .unwrap_or(1.0)
        .min(1.0)
}

fn normal(p1: &Pos, s1: &Size, p2: &Pos, s2: &Size) -> Option<Vel> {
    let m1 = *p1 + *s1 / 2.0;
    let m1 = Isometry::translation(m1.x, m1.y);
    let c1 = Cuboid::new((*s1 / 2.0).to_vec());

    let m2 = *p2 + *s2 / 2.0;
    let m2 = Isometry::translation(m2.x, m2.y);
    let c2 = Cuboid::new((*s2 / 2.0).to_vec());

    contact(&m1, &c1, &m2, &c2, 3.0).map(|c| {
        let x = c.normal.as_ref()[0].round();
        let y = c.normal.as_ref()[1].round();
        Vel::new(x, y)
    })
}

fn cease_vel(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size) -> Vel {
    let vel = match normal(p1, s1, p2, s2) {
        Some(n) => {
            let mut v = v1.clone();

            if n.x * v1.x > 0.0 {
                v.x = 0.0;
            }
            if n.y * v1.y > 0.0 {
                v.x *= 0.9;
                v.y = 0.0;
            }

            v
        }
        None => Vel::zero(),
    };

    vel
}

fn update_vel(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size, v2: &Vel) -> (Vel, Vel) {
    let toi = toi(p1, s1, v1, p2, s2, v2);

    if toi == 0.0 {
        (cease_vel(p1, s1, v1, p2, s2), cease_vel(p2, s2, v2, p1, s1))
    } else {
        (*v1 * toi, *v2 * toi)
    }
}

fn collide(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size, v2: &Vel) -> bool {
    toi(p1, s1, v1, p2, s2, v2) < 1.0
}

fn min(v1: &Vel, v2: &Vel) -> Vel {
    let len = |p: &Vel| p.x * p.x + p.y * p.y;

    if len(v1) < len(v2) {
        v1.clone()
    } else {
        v2.clone()
    }
}

impl<'a> System<'a> for UpdateCollide {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Enemy>,
        ReadStorage<'a, Boss>,
        ReadStorage<'a, Block>,
        Read<'a, User>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (e, pos, siz, mut vel, bullet, mut ply, enemy, boss, blk, user, lazy): Self::SystemData) {
        let mut map = HashMap::<_, Vel>::new();

        for (e1, p1, s1, v1, _) in (&e, &pos, &siz, &vel, &ply).join() {
            for (e2, p2, s2, v2, _) in (&e, &pos, &siz, &vel, &blk).join() {
                let (v1, v2) = update_vel(p1, s1, v1, p2, s2, v2);

                if let Some(vel) = map.get_mut(&e1) {
                    *vel = min(vel, &v1);
                } else {
                    map.insert(e1, v1);
                }
                if let Some(vel) = map.get_mut(&e2) {
                    *vel = min(vel, &v2);
                } else {
                    map.insert(e2, v2);
                }
            }
        }

        // bullet v.s. enemies
        for (e1, p1, s1, v1, _) in (&e, &pos, &siz, &vel, &bullet).join() {
            for (e2, p2, s2, v2, player) in (&e, &pos, &siz, &vel, &enemy).join() {
                if boss.get(e2).is_some() {
                    // boss never die
                    continue;
                }

                if collide(p1, s1, v1, p2, s2, v2) {
                    e.delete(e1);
                    e.delete(e2);
                }
            }
        }

        for (e1, p1, s1, v1, ply) in (&e, &pos, &siz, &vel, &mut ply).join() {
            for (e2, p2, s2, v2, _) in (&e, &pos, &siz, &vel, &enemy).join() {
                if !user.is_me(ply) {
                    continue;
                }

                if collide(p1, s1, v1, p2, s2, v2) {
                    ply.lives = ply.lives.saturating_sub(1);

                    if ply.lives == 0 {
                        e.delete(e1);

                        // Game over!
                        lazy.create_entity(&e)
                            .with(Vel::zero())
                            .with(Pos::new(-300.0, 0.0))
                            .with(Acc::zero())
                            .with(Size::new(600.0, 600.0))
                            .with(Asset(900))
                            .build();
                    }

                    if boss.get(e2).is_some() {
                        // boss never die
                        continue;
                    }

                    e.delete(e2);
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
    type SystemData = (Entities<'a>,
                       ReadStorage<'a, Pos>,
                       ReadStorage<'a, Player>,
                       ReadStorage<'a, GC>,
                       Read<'a, User>);

    fn run(&mut self, (e, pos, ply, gc, user): Self::SystemData) {
        for (plypos, ply) in (&pos, &ply).join() {
            if !user.is_me(ply) {
                continue;
            }

            for (e1, pos, gc) in (&e, &pos, &gc).join() {
                let d = *pos - *plypos;
                if (d.x * d.x + d.y * d.y).sqrt() >= 2000.0 {
                    e.delete(e1);
                }
            }
        }
        // TODO: Remove entities which are out of screen
        // e.delete(lz);
    }
}

pub struct Systems {
    world: World,
    io: Io,
}

impl Systems {
    pub fn new(cfg: Config) -> Result<Self> {
        let mut io = Io::new(cfg)?;
        let mut world = World::new();

        world.register::<Pos>();
        world.register::<Vel>();
        world.register::<Acc>();
        world.register::<Size>();
        world.register::<Player>();
        world.register::<Enemy>();
        world.register::<Boss>();
        world.register::<GC>();
        world.register::<Gun>();
        world.register::<Bullet>();
        world.register::<Landmark>();
        world.register::<Block>();
        world.register::<Dir>();
        world.register::<Asset>();
        world.insert(Action::default());
        world.insert(User::default());

        info!("Requesting terrain data");

        for t in io.get_all_terrain()? {
            info!("Received terrain from server: {:?}", t);
            world
                .create_entity()
                .with(Block)
                .with(Acc::new(0.0, 0.0))
                .with(Vel::new(0.0, 0.0))
                .with(t.asset)
                .with(t.pos)
                .with(t.size)
                .build();
        }

        if io.is_client() {
            info!("Logging in");

            let info = LoginAck{
                player: Player {
                    id: 1,
                    cls: CLASS_CHIBA,
                    lives: 10,
                },
                spawn: Pos::new(100.0, 100.0)
            }; // io.login(CLASS_CHIBA)?;

            {
                let mut user = world.write_resource::<User>();
                user.set(&info.player);
            }

            world
                .create_entity()
                .with(Size::new(50.0, 40.0))
                .with(Vel::zero())
                .with(Acc::new(0.0, -0.15))
                .with(info.spawn)
                .with(info.player)
                .with(Asset(1))
                .with(Dir(1.0))
                .build();
        }

        for i in 2..100 {
            world
                .create_entity()
                .with(Size::new(50.0, 50.0))
                .with(Vel::zero())
                .with(Acc::new(0.0, -0.15))
                .with(Pos::new(i as f32 * 100.0, 500.0))
                .with(Player {
                    id: i,
                    cls: CLASS_SAITAMA,
                    lives: 10,
                })
                .with(Enemy)
                .with(Asset(3))
                .with(Dir(-1.0))
                .build();
        }

        world
            .create_entity()
            .with(Size::new(594.0 * 0.8, 327.0 * 0.8))
            .with(Vel::new(-1.0, 0.0))
            .with(Acc::zero())
            .with(Pos::new(2000.0, 200.0))
            .with(Enemy)
            .with(Boss { step: 0 })
            .with(Asset(4))
            .with(Dir(-1.0))
            .build();

        Ok(Self { world, io })
    }

    pub fn update(&mut self) {
        Print.run_now(&mut self.world);
        Input::new(&mut self.io).run_now(&mut self.world);
        BjarneSystem.run_now(&mut self.world);
        UpdateVel.run_now(&mut self.world);
        UpdateCollide.run_now(&mut self.world);
        UpdatePos.run_now(&mut self.world);
        OutOfBound.run_now(&mut self.world);
        Print.run_now(&mut self.world);
        self.world.maintain();
    }

    pub fn render<'a, T: System<'a>>(&'a mut self, mut sys: T) {
        sys.run_now(&mut self.world);
    }

    pub fn fetch_action<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Action),
    {
        let mut action = self.world.write_resource();

        f(&mut action);
    }
}

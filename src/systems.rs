use crate::{
    client::Client, components::*, config::Config, error::Result, protocol::*, resources::*,
};
use ncollide2d::{
    math::Isometry,
    query::{contact, time_of_impact},
};
use std::collections::HashMap;

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Color},
    lifecycle::Window,
};

use specs::prelude::*;
use specs_derive::Component;

struct Print;

impl<'a> System<'a> for Print {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>);

    fn run(&mut self, (e, pos): Self::SystemData) {
        for (e, pos) in (&e, &pos).join() {
            println!("{:?}: pos={:?}", e, pos);
        }
    }
}

struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (
        Entities<'a>,
        Write<'a, Action>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Dir>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (e, mut act, player, pos, siz, mut vel, mut dir, lazy): Self::SystemData) {
        for (_, pos, siz, vel, dir) in (&player, &pos, &siz, &mut vel, &mut dir).join() {
            if act.jump {
                vel.0.y = 5.0;
            }
            if act.right {
                vel.0.x = 5.0;
                dir.0 = 1.0;
            }
            if act.left {
                vel.0.x = -5.0;
                dir.0 = -1.0;
            }
            if act.take {
                let d = if dir.0 > 0.0 { siz.0.x } else { -10.0 };
                lazy.create_entity(&e)
                    .with(Vel::new(10.0 * dir.0, 0.0))
                    .with(Pos(pos.0 + Vector::new(d, 0.0)))
                    .with(Acc::zero())
                    .with(Bullet { class: CLASS_CHIBA })
                    .with(Size::new(10.0, 10.0))
                    .build();
            }
        }

        act.clear();
    }
}

struct UpdateVel;

impl<'a> System<'a> for UpdateVel {
    type SystemData = (WriteStorage<'a, Vel>, ReadStorage<'a, Acc>);

    fn run(&mut self, (mut vel, acc): Self::SystemData) {
        for (vel, acc) in (&mut vel, &acc).join() {
            vel.0 += acc.0;
        }
    }
}

struct UpdateCollide;

fn toi(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size, v2: &Vel) -> f32 {
    let m1 = p1.0 + s1.0 / 2.0;
    let m1 = Isometry::translation(m1.x, m1.y);
    let c1 = Rectangle::new_sized(s1.0).into_cuboid();
    let v1 = v1.0.into_vector();

    let m2 = p2.0 + s2.0 / 2.0;
    let m2 = Isometry::translation(m2.x, m2.y);
    let c2 = Rectangle::new_sized(s2.0).into_cuboid();
    let v2 = v2.0.into_vector();

    time_of_impact(&m1, &v1, &c1, &m2, &v2, &c2)
        .unwrap_or(1.0)
        .min(1.0)
}

fn normal(p1: &Pos, s1: &Size, p2: &Pos, s2: &Size) -> Option<Vector> {
    let m1 = p1.0 + s1.0 / 2.0;
    let m1 = Isometry::translation(m1.x, m1.y);
    let c1 = Rectangle::new_sized(s1.0).into_cuboid();

    let m2 = p2.0 + s2.0 / 2.0;
    let m2 = Isometry::translation(m2.x, m2.y);
    let c2 = Rectangle::new_sized(s2.0).into_cuboid();

    contact(&m1, &c1, &m2, &c2, 3.0).map(|c| {
        let x = c.normal.as_ref()[0].round();
        let y = c.normal.as_ref()[1].round();
        Vector::new(x, y)
    })
}

fn cease_vel(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size) -> Vel {
    let mut vel = v1.clone();

    let vel = match normal(p1, s1, p2, s2) {
        Some(n) => {
            let mut v = v1.clone();

            if n.x * v1.0.x > 0.0 {
                v.0.x = 0.0;
            }
            if n.y * v1.0.y > 0.0 {
                v.0.x *= 0.9;
                v.0.y = 0.0;
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
        (Vel(v1.0 * toi), Vel(v2.0 * toi))
    }
}

fn min(v1: &Vel, v2: &Vel) -> Vel {
    let len = |p: &Vel| p.0.x * p.0.x + p.0.y * p.0.y;

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
        ReadStorage<'a, Player>,
        ReadStorage<'a, Block>,
    );

    fn run(&mut self, (e, pos, siz, mut vel, ply, blk): Self::SystemData) {
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
            pos.0 += vel.0;
        }
    }
}

struct Render<'a> {
    window: &'a mut Window,
}

impl<'a> Render<'a> {
    fn new(window: &'a mut Window) -> Self {
        Self { window }
    }
}

impl<'a, 'b> System<'a> for Render<'b> {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Bullet>,
        ReadStorage<'a, Block>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Dir>,
    );

    fn run(&mut self, (e, pos, siz, bullet, block, player, dir): Self::SystemData) {
        let center = self.window.screen_size() / 2.0;
        let center = Pos::new(center.x, 0.0);
        let mut origin = Pos::new(0.0, 0.0);
        for (pos, _) in (&pos, &player).join() {
            origin = Pos::new(pos.0.x, 0.0);
        }

        self.window.clear(Color::WHITE).unwrap();

        let size = self.window.screen_size();
        let mut drw = |pos: Vector, siz: Vector, col| {
            let pos = (Vector::new(pos.x, size.y - pos.y - siz.y) - origin.0) + center.0;
            let siz = Vector::new(siz.x, siz.y);
            self.window.draw(&Rectangle::new(pos, siz), col);
        };

        for (e, pos, siz) in (&e, &pos, &siz).join() {
            let col = if player.get(e).is_some() {
                Col(Color::GREEN)
            } else if bullet.get(e).is_some() {
                Col(Color::BLACK)
            } else if block.get(e).is_some() {
                Col(Color::BLUE)
            } else {
                Col(Color::RED)
            };

            drw(pos.0, siz.0, col);

            if player.get(e).is_some() {
                let d = dir.get(e).unwrap_or(&Dir(1.0));
                let d = if d.0 > 0.0 { siz.0.x } else { -10.0 };
                drw(
                    pos.0 + Vector::new(d, 0.0),
                    Vector::new(10.0, 10.0),
                    Col(Color::BLACK),
                );
            }
        }
    }
}

struct OutOfBound;

impl<'a> System<'a> for OutOfBound {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (e, lazy): Self::SystemData) {
        // TODO: Remove entities which are out of screen
        // e.delete(lz);
    }
}

pub fn run_world() {
    let mut sys = Systems::new(Config::default()).unwrap();

    for t in 0..10 {
        println!("---- time {} ----", t);
        sys.update();
    }
}

pub struct Systems {
    world: World,
    game_client: Client,
    terrain_client: Client,
}

impl Systems {
    pub fn new(cfg: Config) -> Result<Self> {
        let game_client = Client::new(&cfg.game_server)?;
        let terrain_client = Client::new(&cfg.terrain_server)?;

        let mut world = World::new();

        world.register::<Pos>();
        world.register::<Vel>();
        world.register::<Acc>();
        world.register::<Size>();
        world.register::<Player>();
        world.register::<Enemy>();
        world.register::<Gun>();
        world.register::<Bullet>();
        world.register::<Landmark>();
        world.register::<Block>();
        world.register::<Dir>();
        world.insert(Action::default());

        for i in 0..10 {
            world
                .create_entity()
                .with(Vel::new(1.0, 2.0))
                .with(Pos::new(160.0, 100.0 * i as f32))
                .with(Acc::new(0.0, -0.05))
                .with(Size::new(10.0, 10.0))
                .build();
            world
                .create_entity()
                .with(Vel::new(1.0, 0.1 * i as f32))
                .with(Pos::new(180.0, 120.0 * i as f32))
                .with(Acc::new(0.0, -0.02))
                .with(Size::new(12.0, 12.0))
                .build();
        }

        world
            .create_entity()
            .with(Pos::new(150.0, 500.0))
            .with(Size::new(30.0, 50.0))
            .with(Vel::zero())
            .with(Acc::new(0.0, -0.15))
            .with(Player { lives: 100 })
            .with(Dir(1.0))
            .build();
        world
            .create_entity()
            .with(Pos::new(300.0, 500.0))
            .with(Size::new(800.0, 150.0))
            .with(Acc::new(0.0, 0.0))
            .with(Vel::zero())
            .with(Block)
            .build();
        world
            .create_entity()
            .with(Pos::new(600.0, 350.0))
            .with(Size::new(400.0, 100.0))
            .with(Acc::new(0.0, 0.0))
            .with(Vel::zero())
            .with(Block)
            .build();
        world
            .create_entity()
            .with(Pos::new(400.0, 200.0))
            .with(Size::new(300.0, 50.0))
            .with(Acc::new(0.0, 0.0))
            .with(Vel::zero())
            .with(Block)
            .build();
        world
            .create_entity()
            .with(Block)
            .with(Acc::new(0.0, 0.0))
            .with(Vel::new(0.0, 0.0))
            .with(Pos::new(0.0, 0.0))
            .with(Size::new(1000.0, 100.0))
            .build();
        world
            .create_entity()
            .with(Block)
            .with(Acc::new(0.0, 0.0))
            .with(Vel::new(0.0, 0.0))
            .with(Pos::new(0.0, 100.0))
            .with(Size::new(100.0, 1000.0))
            .build();

        Ok(Self {
            world,
            game_client,
            terrain_client,
        })
    }

    pub fn update(&mut self) {
        Print.run_now(&mut self.world);
        Input.run_now(&mut self.world);
        UpdateVel.run_now(&mut self.world);
        UpdateCollide.run_now(&mut self.world);
        UpdatePos.run_now(&mut self.world);
        OutOfBound.run_now(&mut self.world);
        Print.run_now(&mut self.world);
    }

    pub fn render(&mut self, window: &mut Window) {
        self.world.maintain();
        Render::new(window).run_now(&mut self.world);
    }

    pub fn fetch_action<F>(&mut self, f: F)
    where
        F: FnOnce(&mut Action),
    {
        let mut action = self.world.write_resource();

        f(&mut action);

        self.game_client
            .send(Message::SendAction(SendAction {
                id: 0,
                action: action.clone(),
            }))
            .unwrap();
    }
}

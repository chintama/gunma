use crate::{components::*, resources::*};
use ncollide2d::{math::Isometry, query::time_of_impact};

use quicksilver::{
    geom::{Rectangle, Shape, Vector},
    graphics::{Background::Col, Color},
    lifecycle::Window,
    Result,
};

use specs::prelude::*;
use specs_derive::Component;

struct Print;

impl<'a> System<'a> for Print {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>);

    fn run(&mut self, (e, pos): Self::SystemData) {
        for (e, pos) in (&e, &pos).join() {
            // println!("{:?}: pos={:?}", e, pos);
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
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Collide>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (e, mut act, player, pos, mut vel, col, lazy): Self::SystemData) {
        for (_, pos, vel, col) in (&player, &pos, &mut vel, &col).join() {
            let movable = col.pos.map(|(_, toi)| toi == 0.0).unwrap_or(true);

            if !movable {
                continue;
            }

            if act.jump {
                vel.0.y = 5.0;
            }
            if act.right {
                vel.0.x = 5.0;
            }
            if act.left {
                vel.0.x = -5.0;
            }
            if act.take {
                lazy.create_entity(&e)
                    .with(Vel::new(10.0, 0.0))
                    .with(pos.clone())
                    .with(Acc::zero())
                    .with(Bullet { class: CLASS_CHIBA })
                    .with(Size::new(10.0, 10.0))
                    .build();
            }
        }

        act.clear();
    }
}

struct UpdateCollide;

impl<'a> System<'a> for UpdateCollide {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Vel>,
        WriteStorage<'a, Collide>,
    );

    fn run(&mut self, (e, pos, siz, vel, mut col): Self::SystemData) {
        for (e1, pos1, siz1, vel1, col) in (&e, &pos, &siz, &vel, &mut col).join() {
            for (e2, pos2, siz2, vel2) in (&e, &pos, &siz, &vel).join() {
                if e1 == e2 {
                    continue;
                }

                let p1 = Isometry::identity();
                let c1 = Rectangle::new_sized(siz1.0).into_cuboid();
                let v1 = vel1.0.into_vector();

                let df = (pos2.0 - pos1.0) + (siz2.0 - siz1.0) / 2.0;
                let p2 = Isometry::translation(df.x, df.y);
                let c2 = Rectangle::new_sized(siz2.0).into_cuboid();
                let v2 = vel2.0.into_vector();

                let colpos = time_of_impact(&p1, &v1, &c1, &p2, &v2, &c2).and_then(|toi| {
                    if toi <= 1.0 {
                        let vel = Vector::new(vel1.0.x * toi, vel1.0.y * toi);
                        Some((pos1.0 + vel, toi))
                    } else {
                        None
                    }
                });

                col.pos = col
                    .pos
                    .and_then(|c1| {
                        let c2 = colpos?;

                        if c1.1 < c2.1 {
                            Some(c1)
                        } else {
                            Some(c2)
                        }
                    })
                    .or(col.pos)
                    .or(colpos);
            }
        }
    }
}

struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Acc>,
        WriteStorage<'a, Collide>,
    );

    fn run(&mut self, (e, mut pos, siz, mut vel, mut acc, mut col): Self::SystemData) {
        for (e, pos, siz, vel, acc) in (&e, &mut pos, &siz, &mut vel, &mut acc).join() {
            // Update location based on client predicition

            pos.0 = col
                .get_mut(e)
                .and_then(|c| c.pos.take())
                .map(|(p, t)| {
                    *vel = Vel::zero();
                    p
                })
                .unwrap_or_else(|| {
                    let newpos = pos.0 + vel.0;
                    vel.0 += acc.0;
                    newpos
                });

            // TODO: Check server reconciliation result
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
    );

    fn run(&mut self, (e, pos, siz, bullet, block, player): Self::SystemData) {
        self.window.clear(Color::WHITE).unwrap();

        let size = self.window.screen_size();
        let mut drw = |pos: Vector, siz: Vector, col| {
            let pos = Vector::new(pos.x, size.y - pos.y - siz.y);
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
    let mut sys = Systems::new();

    for t in 0..10 {
        println!("---- time {} ----", t);
        sys.update();
    }
}

pub struct Systems {
    world: World,
}

impl Systems {
    pub fn new() -> Self {
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
        world.register::<Collide>();
        world.insert(Action::default());

        // for i in 0..10 {
        //     world
        //         .create_entity()
        //         .with(Vel::new(1.0, 2.0))
        //         .with(Pos::new(100.0, 100.0 * i as f32))
        //         .with(Acc::new(0.0, -0.05))
        //         .with(Size::new(10.0, 10.0))
        //         .with(Collide::new())
        //         .build();
        //     world
        //         .create_entity()
        //         .with(Vel::new(1.0, 0.1 * i as f32))
        //         .with(Pos::new(100.0, 120.0 * i as f32))
        //         .with(Acc::new(0.0, -0.02))
        //         .with(Size::new(12.0, 12.0))
        //         .with(Collide::new())
        //         .build();
        // }

        world
            .create_entity()
            .with(Acc::new(0.0, -0.1))
            .with(Vel::new(3.0, 5.0))
            .with(Pos::new(200.0, 200.0))
            .with(Size::new(20.0, 20.0))
            .with(Collide::new())
            .build();
        world
            .create_entity()
            .with(Pos::new(100.0, 500.0))
            .with(Size::new(10.0, 30.0))
            .with(Vel::zero())
            .with(Acc::new(0.0, -0.15))
            .with(Player { lives: 100 })
            .with(Collide::new())
            .build();
        world
            .create_entity()
            .with(Block)
            .with(Acc::new(0.0, 0.0))
            .with(Vel::new(0.0, 0.0))
            .with(Pos::new(0.0, 0.0))
            .with(Size::new(1000.0, 20.0))
            .with(Collide::new())
            .build();
        world
            .create_entity()
            .with(Block)
            .with(Acc::new(0.0, 0.0))
            .with(Vel::new(0.0, 0.0))
            .with(Pos::new(200.0, 200.0))
            .with(Size::new(200.0, 20.0))
            .with(Collide::new())
            .build();

        Self { world }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn update(&mut self) {
        Print.run_now(&mut self.world);
        UpdateCollide.run_now(&mut self.world);
        Input.run_now(&mut self.world);
        UpdatePos.run_now(&mut self.world);
        OutOfBound.run_now(&mut self.world);
        Print.run_now(&mut self.world);
    }

    pub fn render(&mut self, window: &mut Window) {
        self.world.maintain();
        Render::new(window).run_now(&mut self.world);
    }
}

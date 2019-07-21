use crate::{components::*, resources::*};

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
        WriteStorage<'a, Vel>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (e, mut act, player, pos, mut vel, lazy): Self::SystemData) {
        for (_, pos, vel) in (&player, &pos, &mut vel).join() {
            if act.jump {
                vel.0.y = 5.0; //Vel::new(0.0, 5.0);
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

struct Update;

impl<'a> System<'a> for Update {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Acc>,
        ReadStorage<'a, Block>,
    );

    fn run(&mut self, (e, mut pos, siz, mut vel, mut acc, blk): Self::SystemData) {
        for blk in blk.join() {
            for (e, pos, siz, vel, acc) in (&e, &mut pos, &siz, &mut vel, &mut acc).join() {
                // Update location based on client predicition
                pos.0 += vel.0;
                vel.0 += acc.0;

                // TODO: Imporve collision check
                if blk.0.overlaps(&Rectangle::new(pos.0, siz.0)) {
                    // Friction
                    vel.0.x /= 1.2;
                    if vel.0.x.abs() < 0.1 {
                        vel.0.x = 0.0;
                    }
                    vel.0.y = 0.0;
                    pos.0 = Vector::new(pos.0.x, blk.0.pos.y + blk.0.size.y);
                }

                // TODO: Check server reconciliation result
            }
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

    fn run(&mut self, (e, pos, siz, bullet, blk, player): Self::SystemData) {
        self.window.clear(Color::WHITE).unwrap();

        let size = self.window.screen_size();
        let mut drw = |pos: Vector, siz: Vector, col| {
            let pos = Vector::new(pos.x, size.y - pos.y);
            let siz = Vector::new(siz.x, siz.y * -1.0);
            self.window.draw(&Rectangle::new(pos, siz), col);
        };

        for blk in blk.join() {
            drw(blk.0.pos, blk.0.size, Col(Color::BLUE));
        }

        for (e, pos, siz) in (&e, &pos, &siz).join() {
            let col = if player.get(e).is_some() {
                Col(Color::GREEN)
            } else if bullet.get(e).is_some() {
                Col(Color::BLACK)
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
        world.insert(Action::default());

        for i in 0..10 {
            world
                .create_entity()
                .with(Vel::new(1.0, 2.0))
                .with(Pos::new(100.0, 100.0 * i as f32))
                .with(Acc::new(0.0, -0.05))
                .with(Size::new(10.0, 10.0))
                .build();
            world
                .create_entity()
                .with(Vel::new(1.0, 0.1 * i as f32))
                .with(Pos::new(100.0, 120.0 * i as f32))
                .with(Acc::new(0.0, -0.02))
                .with(Size::new(12.0, 12.0))
                .build();
        }

        world
            .create_entity()
            .with(Acc::new(0.0, -0.1))
            .with(Vel::new(3.0, 5.0))
            .with(Pos::new(200.0, 200.0))
            .with(Size::new(20.0, 20.0))
            .build();
        world
            .create_entity()
            .with(Pos::new(100.0, 500.0))
            .with(Size::new(15.0, 15.0))
            .with(Vel::zero())
            .with(Acc::new(0.0, -0.15))
            .with(Player { lives: 100 })
            .build();
        world
            .create_entity()
            .with(Block::new(0.0, 0.0, 1000.0, 50.0))
            .build();

        Self { world }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn update(&mut self) {
        Print.run_now(&mut self.world);
        Input.run_now(&mut self.world);
        Update.run_now(&mut self.world);
        OutOfBound.run_now(&mut self.world);
        Print.run_now(&mut self.world);
    }

    pub fn render(&mut self, window: &mut Window) {
        self.world.maintain();
        Render::new(window).run_now(&mut self.world);
    }
}

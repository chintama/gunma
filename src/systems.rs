use crate::{components::*, resources::*};

use quicksilver::{
    geom::{Rectangle, Vector},
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

struct Update;

impl<'a> System<'a> for Update {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
    );

    fn run(&mut self, (e, mut pos, mut vel, acc): Self::SystemData) {
        for (e, pos, vel) in (&e, &mut pos, &mut vel).join() {
            // Update location based on client predicition
            pos.0 += vel.0;
            vel.0 += acc.get(e).unwrap_or(&Acc::new(0.0, 0.0)).0;

            // TODO: Check collision

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
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>);

    fn run(&mut self, (e, pos): Self::SystemData) {
        self.window.clear(Color::WHITE).unwrap();

        for (e, pos) in (&e, &pos).join() {
            self.window
                .draw(&Rectangle::new(pos.0, (50, 50)), Col(Color::RED));
        }
    }
}

struct Spawn;

impl<'a> System<'a> for Spawn {
    type SystemData = (Entities<'a>, Read<'a, LazyUpdate>);

    fn run(&mut self, (e, lazy): Self::SystemData) {
        // TODO: Remove this system
        // This is just an example to show how to spawn/delete

        let lz = lazy
            .create_entity(&e)
            .with(Vel::new(1.0, 0.0))
            .with(Pos::new(0.0, 0.0))
            .build();
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
        world.register::<Player>();
        world.register::<Enemy>();
        world.register::<Gun>();
        world.register::<Bullet>();
        world.register::<Landmark>();
        world.add_resource(Action::default());

        world
            .create_entity()
            .with(Vel::new(1.0, 2.0))
            .with(Pos::new(3.0, 3.0))
            .build();
        world
            .create_entity()
            .with(Vel::new(2.0, 0.0))
            .with(Pos::new(0.0, 0.0))
            .build();
        world
            .create_entity()
            .with(Acc::new(0.1, 0.1))
            .with(Vel::new(2.0, 0.0))
            .with(Pos::new(0.0, 0.0))
            .build();
        world.create_entity().with(Pos::new(10.0, 10.0)).build();

        Self { world }
    }

    pub fn world(&self) -> &World {
        &self.world
    }

    pub fn update(&mut self) {
        Print.run_now(&mut self.world);
        Update.run_now(&mut self.world);
        Spawn.run_now(&mut self.world);
        Print.run_now(&mut self.world);
    }

    pub fn render(&mut self, window: &mut Window) {
        Render::new(window).run_now(&mut self.world);
    }
}

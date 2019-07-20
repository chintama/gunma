use crate::components::*;
use quicksilver::geom::Vector;
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

struct Render;

impl<'a> System<'a> for Render {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>);

    fn run(&mut self, (e, pos): Self::SystemData) {
        for (e, pos) in (&e, &pos).join() {
            // TODO: Render
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

pub fn run_specs() {
    let mut world = World::new();

    let mut dispatcher = DispatcherBuilder::new()
        .with(Print, "preprint", &[])
        .with(Update, "update", &["preprint"])
        .with(Render, "render", &["update"])
        .with(Spawn, "spawn", &["update"])
        .with(Print, "postprint", &["spawn"])
        .build();

    dispatcher.setup(&mut world);

    world
        .create_entity()
        .with(Vel::new(2.0, 0.0))
        .with(Pos::new(0.0, 0.0))
        .build();
    world.create_entity().with(Pos::new(2.0, 0.0)).build();

    for t in 0..10 {
        println!("---- time {} ----", t);
        dispatcher.dispatch(&world);
        world.maintain();
    }
}

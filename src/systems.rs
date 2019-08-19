use crate::{
    client::render::*,
    components::*,
    entities::*,
    error::Result,
    events::*,
    resources::*,
    terrain::{parse_terrain, read_terrain, write_terrain},
};
use specs::prelude::*;

use log::*;

pub mod client;
pub mod common;
pub mod render;
pub mod server;

pub use self::{client::*, common::*, server::*};

pub struct Systems {
    pub(crate) world: World,
}

impl Systems {
    pub fn new() -> Result<Self> {
        let mut world = World::new();

        world.register::<Pos>();
        world.register::<Vel>();
        world.register::<Acc>();
        world.register::<Ori>();
        world.register::<Size>();
        world.register::<AssetId>();
        world.register::<Player>();
        world.register::<Bullet>();
        world.register::<Landmark>();
        world.register::<Block>();

        world.insert(Action::default());
        world.insert(UserEntity::default());
        world.insert(ServerQueue::default());
        world.insert(ClientQueue::default());

        write_assets("assets.json");
        write_terrain("terrain.json", parse_terrain("terrain.txt"));

        let td = read_terrain("terrain.json");
        for t in td {
            if t.blk {
                world.create_entity().create_terrain(t.pos, t.siz, t.aid);
            } else {
                world.create_entity().create_background(t.pos, t.siz, t.aid);
            }
        }

        /*
        for y in 0..4 {
            for x in 0..18 {
                let xx = if x == 0 {
                    0
                } else if x == 17 {
                    3
                } else {
                    1
                };
                let yy = if y == 0 {
                    0
                } else if y == 3 {
                    3
                } else {
                    1
                };
                let aid = AssetId::new(10000 + xx * 100 + yy);

                let (x, y) = (x as f32, y as f32);

                world.create_entity().create_terrain(
                    Pos::new(x * 60.0, 120.0 - y * 60.0),
                    Size::new(60.0, 60.0),
                    aid,
                );
            }
        }

        {
            // clif
            world.create_entity().create_terrain(
                Pos::new(1500.0, 120.0),
                Size::new(60.0, 60.0),
                AssetId::new(10000),
            );
            world.create_entity().create_terrain(
                Pos::new(1500.0, 60.0),
                Size::new(60.0, 60.0),
                AssetId::new(10001),
            );
            world.create_entity().create_terrain(
                Pos::new(1500.0, 0.0),
                Size::new(60.0, 60.0),
                AssetId::new(10001),
            );

            // flat
            world.create_entity().create_terrain(
                Pos::new(1560.0, 120.0),
                Size::new(60.0, 60.0),
                AssetId::new(10100),
            );
            world.create_entity().create_terrain(
                Pos::new(1560.0, 60.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );
            world.create_entity().create_terrain(
                Pos::new(1560.0, 0.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );

            // up
            world.create_entity().create_terrain(
                Pos::new(1620.0, 300.0),
                Size::new(60.0, 60.0),
                AssetId::new(10000),
            );
            world.create_entity().create_terrain(
                Pos::new(1620.0, 240.0),
                Size::new(60.0, 60.0),
                AssetId::new(10001),
            );
            world.create_entity().create_terrain(
                Pos::new(1620.0, 180.0),
                Size::new(60.0, 60.0),
                AssetId::new(10001),
            );
            world.create_entity().create_terrain(
                Pos::new(1620.0, 120.0),
                Size::new(60.0, 60.0),
                AssetId::new(60001),
            );
            world.create_entity().create_terrain(
                Pos::new(1620.0, 60.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );
            world.create_entity().create_terrain(
                Pos::new(1620.0, 0.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );

            // up-flat
            world.create_entity().create_terrain(
                Pos::new(1680.0, 300.0),
                Size::new(60.0, 60.0),
                AssetId::new(10100),
            );
            world.create_entity().create_terrain(
                Pos::new(1680.0, 240.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );
            world.create_entity().create_terrain(
                Pos::new(1680.0, 180.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );
            world.create_entity().create_terrain(
                Pos::new(1680.0, 120.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );
            world.create_entity().create_terrain(
                Pos::new(1680.0, 60.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );
            world.create_entity().create_terrain(
                Pos::new(1680.0, 0.0),
                Size::new(60.0, 60.0),
                AssetId::new(10101),
            );
        }

        world.create_entity().create_terrain(
            Pos::new(400.0, 500.0),
            Size::new(60.0, 60.0),
            AssetId::new(20000),
        );
        world.create_entity().create_terrain(
            Pos::new(460.0, 500.0),
            Size::new(60.0, 60.0),
            AssetId::new(20100),
        );
        world.create_entity().create_terrain(
            Pos::new(520.0, 500.0),
            Size::new(60.0, 60.0),
            AssetId::new(20300),
        );

        world.create_entity().create_background(
            Pos::new(0.0, 180.0),
            Size::new(240.0, 320.0),
            AssetId::new(30000),
        );

        // grass
        {
            world.create_entity().create_background(
                Pos::new(200.0, 180.0),
                Size::new(60.0, 30.0),
                AssetId::new(40000),
            );
            world.create_entity().create_background(
                Pos::new(320.0, 180.0),
                Size::new(60.0, 30.0),
                AssetId::new(40001),
            );
            world.create_entity().create_background(
                Pos::new(440.0, 180.0),
                Size::new(60.0, 30.0),
                AssetId::new(40002),
            );
            world.create_entity().create_background(
                Pos::new(660.0, 180.0),
                Size::new(60.0, 30.0),
                AssetId::new(40003),
            );
            world.create_entity().create_background(
                Pos::new(780.0, 180.0),
                Size::new(60.0, 30.0),
                AssetId::new(40004),
            );
            world.create_entity().create_background(
                Pos::new(900.0, 180.0),
                Size::new(60.0, 30.0),
                AssetId::new(40005),
            );
        }

        // bridge
        {
            // upper
            world.create_entity().create_background(
                Pos::new(0.0, 180.0),
                Size::new(60.0, 40.0),
                AssetId::new(50102),
            );
            world.create_entity().create_background(
                Pos::new(-60.0, 180.0),
                Size::new(60.0, 40.0),
                AssetId::new(50101),
            );
            world.create_entity().create_background(
                Pos::new(-120.0, 180.0),
                Size::new(60.0, 40.0),
                AssetId::new(50101),
            );
            world.create_entity().create_background(
                Pos::new(-180.0, 180.0),
                Size::new(60.0, 40.0),
                AssetId::new(50101),
            );

            world.create_entity().create_background(
                Pos::new(-240.0, 180.0),
                Size::new(60.0, 40.0),
                AssetId::new(50101),
            );
            world.create_entity().create_background(
                Pos::new(-300.0, 180.0),
                Size::new(60.0, 40.0),
                AssetId::new(50100),
            );

            // lower
            world.create_entity().create_terrain(
                Pos::new(0.0, 120.0 + 40.0),
                Size::new(60.0, 20.0),
                AssetId::new(50000),
            );
            world.create_entity().create_terrain(
                Pos::new(-60.0, 120.0 + 40.0),
                Size::new(60.0, 20.0),
                AssetId::new(50000),
            );
            world.create_entity().create_terrain(
                Pos::new(-120.0, 120.0 + 40.0),
                Size::new(60.0, 20.0),
                AssetId::new(50000),
            );
            world.create_entity().create_terrain(
                Pos::new(-180.0, 120.0 + 40.0),
                Size::new(60.0, 20.0),
                AssetId::new(50000),
            );
            world.create_entity().create_terrain(
                Pos::new(-240.0, 120.0 + 40.0),
                Size::new(60.0, 20.0),
                AssetId::new(50000),
            );
            world.create_entity().create_terrain(
                Pos::new(-300.0, 120.0 + 40.0),
                Size::new(60.0, 20.0),
                AssetId::new(50000),
            );
        }
        */

        Ok(Self { world })
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

    pub fn client_login(&mut self, cls: Class) {
        info!("Connecting");
        let queue = wsq::ClientQueue::new("ws://127.0.0.1:8980").unwrap();
        info!("Connected");

        info!("Logging in as {:?}", cls);
        queue.push(Event::Login(Login::new(cls))).unwrap();
        let ack = loop {
            match queue.pop().unwrap() {
                Event::LoginAck(ack) => {
                    info!("Logged in: {:?}", ack);
                    break ack;
                }
                e => {
                    warn!("Discard unexpected message on login: {:?}", e);
                    continue;
                }
            }
        };
        info!("Logged in: {:?}", ack);
        self.world.write_resource::<ClientQueue>().set(queue);

        let entity = self
            .world
            .create_entity()
            .create_player_by_state(ack.ps.clone());

        self.world
            .write_resource::<UserEntity>()
            .set((ack.ps.ply.id, entity));
    }

    pub fn client_update(&mut self, action: Option<Action>) {
        // Get user input if exists
        match action {
            Some(action) => {
                let mut act = self.world.write_resource();
                *act = action;
            }
            None => {}
        }

        TakeAction.run_now(&mut self.world);
        UpdateVel.run_now(&mut self.world);
        ReduceVel.run_now(&mut self.world);
        CheckCollide.run_now(&mut self.world);
        UpdatePos.run_now(&mut self.world);
        Reconcile.run_now(&mut self.world);
        // ApplyCollide.run_now(&mut self.world);
        // info!("UPDATE");
        self.world.maintain();
    }

    pub fn server_start(&mut self) {
        // Connect to server
        let queue = wsq::ServerQueue::new("127.0.0.1:8980").unwrap();

        // Setup
        self.world.write_resource::<ServerQueue>().set(queue);
    }

    pub fn server_update(&mut self) {
        Input.run_now(&mut self.world);
        UpdateVel.run_now(&mut self.world);
        ReduceVel.run_now(&mut self.world);
        CheckCollide.run_now(&mut self.world);
        UpdatePos.run_now(&mut self.world);
        Publish.run_now(&mut self.world);
        self.world.maintain();
    }
}

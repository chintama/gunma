use crate::{components::*, entities::*, error::Result, events::*, resources::*};
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

        world.create_entity().create_terrain(
            Pos::new(0.0, 0.0),
            Size::new(1000.0, 100.0),
            AssetId::new(200),
        );

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

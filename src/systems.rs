use crate::{
    collide::{cease_vel, collide, normal, toi, update_vel},
    components::*,
    config::Config,
    entities::*,
    error::Result,
    events::*,
    resources::*,
    vector::Vector,
};
use specs::{prelude::*, world::EntityBuilder};
use std::collections::HashMap;

use log::*;

mod common;
mod print;
mod user;

pub use self::{common::*, print::*, user::*};

pub struct Systems {
    pub(crate) world: World,
}

impl Systems {
    pub fn new() -> Result<Self> {
        let mut world = World::new();

        world.register::<ObjectId>();
        world.register::<Class>();
        world.register::<Player>();
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
        world.insert(UserEntity::default());
        world.insert(ServerQueue::default());
        world.insert(ClientQueue::default());

        world
            .create_entity()
            .create_terrain(Pos::new(0.0, 0.0), Size::new(1000.0, 100.0));

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
        let mut queue = wsq::ClientQueue::new("ws://127.0.0.1:8980").unwrap();
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

        let entity = self.world.create_entity().create_player(
            ObjectId::new(ack.id, 0),
            Player(0),
            cls,
            ack.pos,
            ack.lives,
            ack.ori,
        );

        self.world
            .write_resource::<UserEntity>()
            .set((ack.id, entity));
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
        let mut queue = wsq::ServerQueue::new("127.0.0.1:8980").unwrap();

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

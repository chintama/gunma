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
        world.insert(UserEntity::default());
        world.insert(ServerQueue::default());
        world.insert(ClientQueue::default());

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

    pub fn client_login(&mut self) {
        let mut queue = wsq::ClientQueue::new("ws://127.0.0.1:8980").unwrap();
        queue.push(Event::Login(Login::new(CLASS_CHIBA))).unwrap();
        let ack = match queue.pop().unwrap() {
            Event::LoginAck(ack) => ack,
            e => panic!("Received unexpected message on login: {:?}", e),
        };
        self.world.write_resource::<ClientQueue>().set(queue);

        let entity = self.world.create_entity()
            .with(Player::new(ack.id))
            .with(CLASS_CHIBA)
            .with(Size::new(50.0, 40.0))
            .with(Asset::new(1))
            .with(Vel::zero())
            .with(Acc::gravity())
            .with(ack.pos)
            .with(ack.lives)
            .with(ack.ori)
            .build();

        self.world.write_resource::<UserEntity>().set((ack.id, entity));
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
        Reconcile.run_now(&mut self.world);
        // ApplyCollide.run_now(&mut self.world);
    }

    pub fn maintain(&mut self) {
        self.world.maintain();
    }
}

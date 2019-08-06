use crate::{
    client::Client,
    collide::{cease_vel, collide, normal, toi, update_vel},
    components::*,
    config::Config,
    entities::*,
    error::Result,
    protocol::*,
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
        world.insert(Events::default());
        world.insert(Network::default());
        // world.insert(PlayerMap::default());

        Ok(Self { world })
    }

    ///
    /// Execute one turn
    ///
    pub fn update(&mut self) {
        Print.run_now(&mut self.world);
        TakeAction.run_now(&mut self.world);
        UpdateVel.run_now(&mut self.world);
        ReduceVel.run_now(&mut self.world);
        CheckCollide.run_now(&mut self.world);
        UpdatePos.run_now(&mut self.world);
        OutOfBound.run_now(&mut self.world);
        Print.run_now(&mut self.world);
        self.world.maintain();
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
}

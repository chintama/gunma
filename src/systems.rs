use crate::{
    client::Client,
    collide::{cease_vel, collide, normal, toi, update_vel},
    components::*,
    config::Config,
    entities::{CreateEntity, EntityCreator},
    error::Result,
    protocol::*,
    resources::*,
    vector::Vector,
};
use specs::{prelude::*, world::EntityBuilder};
use std::collections::HashMap;

use log::*;

struct Print;

impl<'a> System<'a> for Print {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>);

    fn run(&mut self, (e, pos): Self::SystemData) {
        for (e, pos) in (&e, &pos).join() {
            trace!("{:?}: pos={:?}", e, pos);
        }
    }
}

struct TakeAction;

impl<'a> System<'a> for TakeAction {
    type SystemData = (
        Entities<'a>,
        Write<'a, Action>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, User>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Dir>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, mut act, player, user, pos, siz, mut vel, acc, mut dir, lazy): Self::SystemData,
    ) {
        for (player, _, pos, siz, vel, acc, dir) in
            (&player, &user, &pos, &siz, &mut vel, &acc, &mut dir).join()
        {
            if !act.update {
                continue;
            }

            if act.jump {
                vel.y = 5.0;
            }
            if act.right {
                vel.x = 5.0;
                dir.0 = 1.0;
            }
            if act.left {
                vel.x = -5.0;
                dir.0 = -1.0;
            }
            if act.take {
                let d = if dir.0 > 0.0 { siz.x } else { -10.0 };
                lazy.create_entity(&e).create_bullet(
                    Vel::new(10.0 * dir.0, 0.0),
                    *pos + Vel::new(d, 0.0),
                    Bullet::new(player.id, player.class),
                    Size::new(30.0, 30.0),
                    Asset(100),
                );
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
            *vel += *acc;
        }
    }
}

struct UpdateCollide;

impl<'a> System<'a> for UpdateCollide {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Bullet>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, User>,
        ReadStorage<'a, Block>,
        Read<'a, LazyUpdate>,
    );

    fn run(&mut self, (e, pos, siz, mut vel, bullet, mut ply, user, blk, lazy): Self::SystemData) {
        let mut map = HashMap::<_, Vel>::new();

        for (e1, p1, s1, _) in (&e, &pos, &siz, &ply).join() {
            for (e2, p2, s2, _) in (&e, &pos, &siz, &blk).join() {
                let z = Vel::zero();
                let v1 = vel.get(e1).unwrap_or(&z);
                let v2 = vel.get(e2).unwrap_or(&z);

                let (v1, v2) = update_vel(p1, s1, v1, p2, s2, v2);

                if let Some(vel) = map.get_mut(&e1) {
                    *vel = vel.min(&v1);
                } else {
                    map.insert(e1, v1);
                }
                if let Some(vel) = map.get_mut(&e2) {
                    *vel = vel.min(&v2);
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
            *pos += *vel;
        }
    }
}

struct OutOfBound;

impl<'a> System<'a> for OutOfBound {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, User>,
    );

    fn run(&mut self, (e, pos, ply, user): Self::SystemData) {
        for (plypos, ply, _) in (&pos, &ply, &user).join() {
            for (e1, pos) in (&e, &pos).join() {
                let d = *pos - *plypos;
                if d.len() >= 2000.0 {
                    e.delete(e1);
                }
            }
        }
    }
}

pub struct Systems {
    pub(crate) world: World,
}

impl Systems {
    pub fn new() -> Result<Self> {
        let mut world = World::new();

        world.register::<Pos>();
        world.register::<Vel>();
        world.register::<Acc>();
        world.register::<Size>();
        world.register::<Player>();
        world.register::<Bullet>();
        world.register::<Landmark>();
        world.register::<Block>();
        world.register::<Background>();
        world.register::<Dir>();
        world.register::<Asset>();
        world.register::<User>();
        world.insert(Action::default());
        world.insert(PlayerUpdates::default());
        world.insert(Events::default());

        Ok(Self { world })
    }

    ///
    /// Add an action for the current user
    ///
    pub fn add_action(&mut self, act: Action) {
        let mut action = self.world.write_resource::<Action>();
        *action = act;
    }

    ///
    /// Update player state
    ///
    pub fn update_player(&mut self, pos: Pos, player: Player, dir: Dir, vel: Vel, acc: Acc) {
        let mut updates = self.world.write_resource::<PlayerUpdates>();
        updates
            .0
            .insert(player.id, PlayerUpdate::new(pos, player, dir, vel, acc));
    }

    ///
    /// Create a new entity
    ///
    pub fn create_entity(&mut self) -> EntityCreator<EntityBuilder> {
        self.world.create_entity().into()
    }

    ///
    /// Execute one turn
    ///
    pub fn update(&mut self) {
        Print.run_now(&mut self.world);
        TakeAction.run_now(&mut self.world);
        UpdateVel.run_now(&mut self.world);
        UpdateCollide.run_now(&mut self.world);
        UpdatePos.run_now(&mut self.world);
        OutOfBound.run_now(&mut self.world);
        Print.run_now(&mut self.world);
        self.world.maintain();
    }

    ///
    /// Retrieve all events happened in the last turn
    ///
    pub fn take_events(&mut self) -> Vec<Event> {
        let mut events = self.world.write_resource::<Events>();
        events.0.split_off(0)
    }

    pub fn render<'a, T: System<'a>>(&'a mut self, mut sys: T) {
        sys.run_now(&mut self.world);
    }
}

#![allow(unused)]

use crate::{
    components::*,
    entities::*,
    events::{self, *},
    resources::*,
};
use log::*;
use specs::prelude::*;

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (
        Entities<'a>,
        Write<'a, ServerQueue>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Ori>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, mut queue, mut player, pos, siz, mut vel, acc, mut ori, lazy): Self::SystemData,
    ) {
        // let mut players: HashMap<_, _> = (&pos, &siz, &mut vel, &acc, &mut ori, &mut player)
        //     .join()
        //     .map(|t| ((t.0).0, t))
        //     .collect();

        // for _ in 0..100 {
        //     let item = queue.get().try_pop().unwrap();

        //     match item {
        //         Some((id, Event::Input(input))) => {
        //             debug!("Receive client input: {:?}", input);

        //             let (oid, cls, pos, siz, vel, acc, ori, player) =
        //                 match players.get_mut(&input.id) {
        //                     Some(t) => t,
        //                     None => continue,
        //                 };

        //             if input.action.jump {
        //                 vel.y = 5.0;
        //             }

        //             if input.action.right {
        //                 vel.x = 5.0;
        //                 ori.x = 1.0;
        //             }

        //             if input.action.left {
        //                 vel.x = -5.0;
        //                 ori.x = -1.0;
        //             }

        //             if input.action.take {
        //                 let d = if ori.x > 0.0 { siz.x } else { -10.0 };
        //                 player.0 += 1;
        //                 lazy.create_entity(&e).create_bullet(
        //                     ObjectId::new(oid.0, player.0),
        //                     Class(cls.0),
        //                     **pos + Vel::new(d, 0.0),
        //                     Vel::new(10.0 * ori.x, 0.0),
        //                     Damage(1),
        //                     **ori,
        //                 );
        //             }
        //         }
        //         Some((id, Event::Login(login))) => {
        //             let ack = Event::LoginAck(LoginAck::new(
        //                 id,
        //                 Pos::new(200.0, 200.0),
        //                 Ori::new(1.0, 0.0),
        //                 Lives::new(10),
        //             ));
        //             lazy.create_entity(&e).create_player(
        //                 ObjectId::new(id, 0),
        //                 Player(0),
        //                 login.cls,
        //                 Pos::new(200.0, 200.0),
        //                 Lives::new(10),
        //                 Ori::new(1.0, 0.0),
        //             );
        //             info!("Login accepted: {:?}", ack);
        //             queue.get().push((id, ack)).unwrap();
        //             info!("Pushed");
        //         }
        //         _ => break,
        //     }
        // }
    }
}

pub struct Publish;

impl<'a> System<'a> for Publish {
    type SystemData = (
        Entities<'a>,
        Read<'a, ServerQueue>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        ReadStorage<'a, Ori>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, AssetId>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Bullet>,
    );

    fn run(
        &mut self,
        (e, queue, pos, vel, acc, ori, size, asset, player, bullet): Self::SystemData,
    ) {
        // let ps: Vec<_> = (
        //     &oid, &e, &pos, &vel, &acc, &ori, &size, &asset, &cls, &lives, &player,
        // )
        //     .join()
        //     .map(
        //         |(oid, e, pos, vel, acc, ori, size, asset, cls, lives, player)| {
        //             PlayerState::new(
        //                 0, *oid, *player, *pos, *vel, *acc, *ori, *size, *asset, *cls, *lives,
        //             )
        //         },
        //     )
        //     .collect();

        // queue.get().push_all(Event::State(State::new(ps))).unwrap();
    }
}

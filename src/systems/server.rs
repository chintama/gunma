use crate::{
    components::*,
    entities::*,
    events::{self, *},
    resources::*,
};
use log::*;
use specs::prelude::*;
use std::collections::HashMap;

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (
        Entities<'a>,
        Write<'a, ServerQueue>,
        ReadStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Ori>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Player>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, mut queue, pos, mut vel, acc, mut ori, siz, mut ply, lazy): Self::SystemData,
    ) {
        let mut players: HashMap<_, _> = (&mut ply, &pos, &mut vel, &acc, &mut ori, &siz)
            .join()
            .map(|t| (t.0.id, t))
            .collect();

        for _ in 0..100 {
            let item = queue.get().try_pop().unwrap();

            match item {
                Some((id, Event::Input(input))) => {
                    debug!("Receive client input: {:?}", input);

                    let (ply, pos, vel, acc, ori, siz) = match players.get_mut(&input.id) {
                        Some(t) => t,
                        None => continue,
                    };

                    if input.act.jump && ply.land {
                        vel.y = 8.0;
                    }

                    if input.act.right {
                        vel.x = 5.0;
                        ori.x = 1.0;
                    }

                    if input.act.left {
                        vel.x = -5.0;
                        ori.x = -1.0;
                    }

                    if input.act.take {
                        let d = if ori.x > 0.0 { siz.x } else { -10.0 };
                        lazy.create_entity(&e).create_bullet(
                            **pos + Vel::new(d, 0.0),
                            Vel::new(10.0 * ori.x, 0.0),
                            **ori,
                            Size::new(30.0, 30.0),
                            AssetId::new(100),
                            ply.bullet(1),
                        );
                    }
                }
                Some((id, Event::Login(login))) => {
                    info!("Received login from channel: {}", id);
                    let ps = PlayerState::new(
                        0,
                        Pos::new(200.0, 2000.0),
                        Vel::zero(),
                        Acc::gravity(),
                        Ori::new(1.0, 0.0),
                        Size::new(50.0, 40.0),
                        AssetId::new(1),
                        Player::spawn(10, login.cls),
                    );

                    let ack = Event::LoginAck(LoginAck::new(ps.clone()));
                    lazy.create_entity(&e).create_player_by_state(ps);

                    info!("Login accepted: {:?}", ack);
                    queue.get().push((id, ack)).unwrap();
                    info!("Pushed");
                }
                _ => break,
            }
        }
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

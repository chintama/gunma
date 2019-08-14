use crate::{
    components::*,
    entities::*,
    events::{self, *},
    resources::*,
};
use log::*;
use specs::prelude::*;
use std::collections::HashMap;

pub struct TakeAction;

impl<'a> System<'a> for TakeAction {
    type SystemData = (
        Entities<'a>,
        Read<'a, UserEntity>,
        Write<'a, Action>,
        Read<'a, ClientQueue>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Class>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Ori>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, user, mut act, queue, player, cls, pos, siz, mut vel, acc, mut ori, lazy): Self::SystemData,
    ) {
        if !act.update {
            return;
        }

        let ep = user.get().1;

        let pos = pos.get(ep).unwrap();
        let siz = siz.get(ep).unwrap();
        let player = player.get(ep).unwrap();
        let cls = cls.get(ep).unwrap();
        let mut vel = vel.get_mut(ep).unwrap();
        let mut ori = ori.get_mut(ep).unwrap();

        if act.jump {
            vel.y = 5.0;
        }

        if act.right {
            vel.x = 5.0;
            ori.x = 1.0;
        }

        if act.left {
            vel.x = -5.0;
            ori.x = -1.0;
        }

        if act.take {
            let d = if ori.x > 0.0 { siz.x } else { -10.0 };
            lazy.create_entity(&e).create_bullet(
                ObjectId::new(user.get().0, 0),
                Class(cls.0),
                *pos + Vel::new(d, 0.0),
                Vel::new(10.0 * ori.x, 0.0),
                Damage(1),
                *ori,
            );
        }

        queue
            .get()
            .push(Event::Input(events::Input::new(
                user.get().0,
                0,
                act.clone(),
            )))
            .unwrap();

        act.clear();
    }
}

pub struct Input;

impl<'a> System<'a> for Input {
    type SystemData = (
        Entities<'a>,
        Write<'a, ServerQueue>,
        ReadStorage<'a, ObjectId>,
        WriteStorage<'a, Player>,
        ReadStorage<'a, Class>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Ori>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, mut queue, oid, mut player, cls, pos, siz, mut vel, acc, mut ori, lazy): Self::SystemData,
    ) {
        let mut players: HashMap<_, _> = (
            &oid,
            &cls,
            &pos,
            &siz,
            &mut vel,
            &acc,
            &mut ori,
            &mut player,
        )
            .join()
            .map(|t| ((t.0).0, t))
            .collect();

        for _ in 0..100 {
            let item = queue.get().try_pop().unwrap();

            match item {
                Some((id, Event::Input(input))) => {
                    debug!("Receive client input: {:?}", input);

                    let (oid, cls, pos, siz, vel, acc, ori, player) =
                        match players.get_mut(&input.id) {
                            Some(t) => t,
                            None => continue,
                        };

                    if input.action.jump {
                        vel.y = 5.0;
                    }

                    if input.action.right {
                        vel.x = 5.0;
                        ori.x = 1.0;
                    }

                    if input.action.left {
                        vel.x = -5.0;
                        ori.x = -1.0;
                    }

                    if input.action.take {
                        let d = if ori.x > 0.0 { siz.x } else { -10.0 };
                        player.0 += 1;
                        lazy.create_entity(&e).create_bullet(
                            ObjectId::new(oid.0, player.0),
                            Class(cls.0),
                            **pos + Vel::new(d, 0.0),
                            Vel::new(10.0 * ori.x, 0.0),
                            Damage(1),
                            **ori,
                        );
                    }
                }
                Some((id, Event::Login(login))) => {
                    let ack = Event::LoginAck(LoginAck::new(
                        id,
                        Pos::new(200.0, 200.0),
                        Ori::new(1.0, 0.0),
                        Lives::new(10),
                    ));
                    lazy.create_entity(&e).create_player(
                        ObjectId::new(id, 0),
                        Player(0),
                        login.cls,
                        Pos::new(200.0, 200.0),
                        Lives::new(10),
                        Ori::new(1.0, 0.0),
                    );
                    info!("Login accepted: {:?}", ack);
                    queue.get().push((id, ack)).unwrap();
                    info!("Pushed");
                }
                _ => break,
            }
        }
    }
}

pub struct UpdateUserVel;

impl<'a> System<'a> for UpdateUserVel {
    type SystemData = (
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        ReadStorage<'a, User>,
    );

    fn run(&mut self, (mut vel, acc, user): Self::SystemData) {
        for (vel, acc, _) in (&mut vel, &acc, &user).join() {
            *vel += *acc;
        }
    }
}

pub struct UpdateUserPos;

impl<'a> System<'a> for UpdateUserPos {
    type SystemData = (
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, User>,
    );

    fn run(&mut self, (mut pos, vel, user): Self::SystemData) {
        for (pos, vel, _) in (&mut pos, &vel, &user).join() {
            *pos += *vel;
        }
    }
}

pub struct OutOfBound;

impl<'a> System<'a> for OutOfBound {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>, ReadStorage<'a, Player>);

    fn run(&mut self, (e, pos, ply): Self::SystemData) {
        // for (plypos, ply, _) in (&pos, &ply, &user).join() {
        //     for (e1, pos) in (&e, &pos).join() {
        //         let d = *pos - *plypos;
        //         if d.len() >= 2000.0 {
        //             e.delete(e1);
        //         }
        //     }
        // }
    }
}

pub struct Reconcile;

impl<'a> System<'a> for Reconcile {
    type SystemData = (
        Entities<'a>,
        Read<'a, ClientQueue>,
        ReadStorage<'a, ObjectId>,
        WriteStorage<'a, Player>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Acc>,
        WriteStorage<'a, Ori>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, Asset>,
        WriteStorage<'a, Class>,
        WriteStorage<'a, Lives>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            e,
            queue,
            oid,
            mut player,
            mut pos,
            mut vel,
            mut acc,
            mut ori,
            mut size,
            mut asset,
            mut cls,
            mut lives,
            lazy,
        ): Self::SystemData,
    ) {
        let state = match queue.get().try_pop().unwrap() {
            Some(Event::State(state)) => state,
            _ => return,
        };

        let mut players: HashMap<_, _> = (
            &oid,
            &e,
            &mut player,
            &mut pos,
            &mut vel,
            &mut acc,
            &mut ori,
            &mut size,
            &mut asset,
            &mut cls,
            &mut lives,
        )
            .join()
            .map(|t| (t.0, t))
            .collect();

        for state in state.players {
            let (oid, e, player, pos, vel, acc, ori, size, asset, cls, lives) =
                match players.get_mut(&state.id) {
                    Some(t) => t,
                    None => {
                        lazy.create_entity(&e).create_player(
                            state.id,
                            state.player,
                            state.cls,
                            state.pos,
                            state.lives,
                            state.ori,
                        );
                        continue;
                    }
                };

            **player = state.player;
            **pos = state.pos;
            **vel = state.vel;
            **acc = state.acc;
            **ori = state.ori;
            **size = state.size;
            **asset = state.asset;
            **cls = state.cls;
            **lives = state.lives;
        }
    }
}

pub struct Publish;

impl<'a> System<'a> for Publish {
    type SystemData = (
        Entities<'a>,
        Read<'a, ServerQueue>,
        ReadStorage<'a, ObjectId>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        ReadStorage<'a, Ori>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Asset>,
        ReadStorage<'a, Class>,
        ReadStorage<'a, Lives>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Bullet>,
    );

    fn run(
        &mut self,
        (e, queue, oid, pos, vel, acc, ori, size, asset, cls, lives, player, bullet): Self::SystemData,
    ) {
        let ps: Vec<_> = (
            &oid, &e, &pos, &vel, &acc, &ori, &size, &asset, &cls, &lives, &player,
        )
            .join()
            .map(
                |(oid, e, pos, vel, acc, ori, size, asset, cls, lives, player)| {
                    PlayerState::new(
                        0, *oid, *player, *pos, *vel, *acc, *ori, *size, *asset, *cls, *lives,
                    )
                },
            )
            .collect();

        queue.get().push_all(Event::State(State::new(ps))).unwrap();
    }
}

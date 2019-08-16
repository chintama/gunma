#![allow(unused)]

use crate::{
    components::*,
    entities::*,
    events::{self, *},
    resources::*,
};
use log::*;
use specs::prelude::*;

pub struct TakeAction;

impl<'a> System<'a> for TakeAction {
    type SystemData = (
        Entities<'a>,
        Read<'a, UserEntity>,
        Write<'a, Action>,
        Read<'a, ClientQueue>,
        ReadStorage<'a, Player>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Size>,
        WriteStorage<'a, Vel>,
        ReadStorage<'a, Acc>,
        WriteStorage<'a, Ori>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (e, user, mut act, queue, player, pos, siz, mut vel, acc, mut ori, lazy): Self::SystemData,
    ) {
        if !act.update {
            return;
        }

        let ep = user.get().1;

        let pos = pos.get(ep).unwrap();
        let siz = siz.get(ep).unwrap();
        let player = player.get(ep).unwrap();
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
                *pos + Vel::new(d, 0.0),
                Vel::new(10.0 * ori.x, 0.0),
                *ori,
                Size::new(30.0, 30.0),
                AssetId::new(2),
                player.bullet(1),
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
        WriteStorage<'a, Player>,
        WriteStorage<'a, Pos>,
        WriteStorage<'a, Vel>,
        WriteStorage<'a, Acc>,
        WriteStorage<'a, Ori>,
        WriteStorage<'a, Size>,
        WriteStorage<'a, AssetId>,
        Read<'a, LazyUpdate>,
    );

    fn run(
        &mut self,
        (
            e,
            queue,
            mut player,
            mut pos,
            mut vel,
            mut acc,
            mut ori,
            mut size,
            mut asset,
            lazy,
        ): Self::SystemData,
    ) {
        // let state = match queue.get().try_pop().unwrap() {
        //     Some(Event::State(state)) => state,
        //     _ => return,
        // };

        // let mut players: HashMap<_, _> = (
        //     &oid,
        //     &e,
        //     &mut player,
        //     &mut pos,
        //     &mut vel,
        //     &mut acc,
        //     &mut ori,
        //     &mut size,
        //     &mut asset,
        //     &mut cls,
        //     &mut lives,
        // )
        //     .join()
        //     .map(|t| (t.0, t))
        //     .collect();

        // for state in state.players {
        //     let (oid, e, player, pos, vel, acc, ori, size, asset, cls, lives) =
        //         match players.get_mut(&state.id) {
        //             Some(t) => t,
        //             None => {
        //                 lazy.create_entity(&e).create_player(
        //                     state.id,
        //                     state.player,
        //                     state.cls,
        //                     state.pos,
        //                     state.lives,
        //                     state.ori,
        //                 );
        //                 continue;
        //             }
        //         };

        //     **player = state.player;
        //     **pos = state.pos;
        //     **vel = state.vel;
        //     **acc = state.acc;
        //     **ori = state.ori;
        //     **size = state.size;
        //     **asset = state.asset;
        //     **cls = state.cls;
        //     **lives = state.lives;
        // }
    }
}

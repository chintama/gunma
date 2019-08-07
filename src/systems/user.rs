use crate::{components::*, entities::*, resources::*, events::Event};
use specs::prelude::*;

pub struct TakeAction;

impl<'a> System<'a> for TakeAction {
    type SystemData = (
        Entities<'a>,
        Read<'a, UserEntity>,
        Write<'a, Action>,
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
        (e, user, mut act, player, cls, pos, siz, mut vel, acc, mut ori, lazy): Self::SystemData,
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
            lazy.create_entity(&e).create(BulletEntity::new(
                *pos + Vel::new(d, 0.0),
                Size::new(30.0, 30.0),
                Vel::new(10.0 * ori.x, 0.0),
                Acc::zero(),
                Asset(100),
                *ori,
                Owner(player.0),
                Class(cls.0),
                Damage(1),
            ));
        }

        act.clear();
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
    type SystemData = (Entities<'a>,
                       Read<'a, ClientQueue>,
                       ReadStorage<'a, Player>,
                       ReadStorage<'a, Bullet>,
                       ReadStorage<'a, Pos>,
                       ReadStorage<'a, Vel>,
                       ReadStorage<'a, Acc>);

    fn run(&mut self, (e, events, pos, ply): Self::SystemData) {
        
    }
}

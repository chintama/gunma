use crate::{components::*, events::PlayerState};
use specs::{
    prelude::*,
    world::{EntityBuilder, LazyBuilder},
};

pub trait CreateEntity<T: Builder>: Sized {
    fn builder(self) -> T;

    fn create_player(
        self,
        pos: Pos,
        vel: Vel,
        acc: Acc,
        ori: Ori,
        siz: Size,
        aid: AssetId,
        ply: Player,
    ) -> Entity {
        self.builder()
            .with(pos)
            .with(vel)
            .with(acc)
            .with(ori)
            .with(siz)
            .with(aid)
            .with(ply)
            .build()
    }

    fn create_player_by_state(self, ps: PlayerState) -> Entity {
        self.create_player(ps.pos, ps.vel, ps.acc, ps.ori, ps.siz, ps.aid, ps.ply)
    }

    fn create_terrain(self, pos: Pos, siz: Size, aid: AssetId) -> Entity {
        self.builder()
            .with(pos)
            .with(siz)
            .with(aid)
            .with(Block)
            .build()
    }

    fn create_background(self, pos: Pos, siz: Size, aid: AssetId) -> Entity {
        self.builder().with(pos).with(siz).with(aid).build()
    }

    fn create_bullet(
        self,
        pos: Pos,
        vel: Vel,
        ori: Ori,
        siz: Size,
        aid: AssetId,
        blt: Bullet,
    ) -> Entity {
        self.builder()
            .with(pos)
            .with(vel)
            .with(ori)
            .with(siz)
            .with(aid)
            .with(blt)
            .build()
    }
}

impl<'a> CreateEntity<EntityBuilder<'a>> for EntityBuilder<'a> {
    fn builder(self) -> Self {
        self
    }
}

impl<'a> CreateEntity<LazyBuilder<'a>> for LazyBuilder<'a> {
    fn builder(self) -> Self {
        self
    }
}

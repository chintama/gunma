use crate::{components::*, systems::Systems};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs::{
    prelude::*,
    world::{EntityBuilder, LazyBuilder},
};

pub trait CreateEntity<T: Builder>: Sized {
    fn builder(self) -> T;

    fn create_player(
        self,
        id: ObjectId,
        player: Player,
        cls: Class,
        pos: Pos,
        lives: Lives,
        ori: Ori,
    ) -> Entity {
        self.builder()
            .with(player)
            .with(Size::new(50.0, 40.0))
            .with(Asset::new(1))
            .with(Vel::zero())
            .with(Acc::gravity())
            .with(id)
            .with(cls)
            .with(pos)
            .with(lives)
            .with(ori)
            .build()
    }

    fn create_terrain(self, pos: Pos, size: Size) -> Entity {
        self.builder()
            .with(Block)
            .with(Background)
            .with(pos)
            .with(size)
            .with(Asset(200))
            .build()
    }

    fn create_bullet(
        self,
        id: ObjectId,
        cls: Class,
        pos: Pos,
        vel: Vel,
        dmg: Damage,
        ori: Ori,
    ) -> Entity {
        self.builder()
            .with(Bullet)
            .with(Size::new(30.0, 30.0))
            .with(Asset::new(100))
            .with(Acc::zero())
            .with(id)
            .with(cls)
            .with(pos)
            .with(vel)
            .with(dmg)
            .with(ori)
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

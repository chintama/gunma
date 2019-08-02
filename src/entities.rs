use crate::{components::*, systems::Systems};
use specs::{
    prelude::*,
    world::{EntityBuilder, LazyBuilder},
};

pub trait CreateEntity<T: Builder>: Sized {
    fn builder(self) -> T;

    fn create_terrain(self, pos: Pos, size: Size, asset: Asset) -> Entity {
        self.builder()
            .with(asset)
            .with(pos)
            .with(size)
            .with(Background)
            .build()
    }

    fn create_terrain_block(self, pos: Pos, size: Size, asset: Asset) -> Entity {
        self.builder()
            .with(asset)
            .with(pos)
            .with(size)
            .with(Background)
            .with(Block)
            .build()
    }

    fn create_user(self, pos: Pos, size: Size, player: Player, asset: Asset) -> Entity {
        self.builder()
            .with(Vel::zero())
            .with(Acc::gravity())
            .with(pos)
            .with(size)
            .with(player)
            .with(asset)
            .with(Dir(1.0))
            .with(User)
            .build()
    }

    fn create_player(self, pos: Pos, size: Size, player: Player, asset: Asset) -> Entity {
        self.builder()
            .with(Vel::zero())
            .with(Acc::gravity())
            .with(pos)
            .with(size)
            .with(player)
            .with(asset)
            .with(Dir(1.0))
            .build()
    }

    fn create_bullet(self, vel: Vel, pos: Pos, bullet: Bullet, size: Size, asset: Asset) -> Entity {
        self.builder()
            .with(vel)
            .with(pos)
            .with(Acc::zero())
            .with(bullet)
            .with(size)
            .with(asset)
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

pub struct EntityCreator<T>(T);

impl<'a, T: Builder> CreateEntity<T> for EntityCreator<T> {
    fn builder(self) -> T {
        self.0
    }
}

impl<'a> From<EntityBuilder<'a>> for EntityCreator<EntityBuilder<'a>> {
    fn from(e: EntityBuilder<'a>) -> Self {
        Self(e)
    }
}

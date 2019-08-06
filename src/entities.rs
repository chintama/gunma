use crate::{components::*, systems::Systems};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs::{
    prelude::*,
    world::{EntityBuilder, LazyBuilder},
};

pub trait BuildEntity<T> {
    fn build(self, builder: T) -> Entity;
}

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct TerrainEntity {
    pub pos: Pos,
    pub size: Size,
    pub asset: Asset,
    pub block: bool,
}

impl<T: Builder> BuildEntity<T> for TerrainEntity {
    fn build(self, builder: T) -> Entity {
        let b = builder
            .with(self.asset)
            .with(self.pos)
            .with(self.size)
            .with(Background);

        if self.block {
            b.with(Block).build()
        } else {
            b.build()
        }
    }
}

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerEntity {
    pub player: Player,
    pub class: Class,
    pub pos: Pos,
    pub size: Size,
    pub vel: Vel,
    pub acc: Acc,
    pub lives: Lives,
    pub asset: Asset,
    pub ori: Ori,
}

impl<T: Builder> BuildEntity<T> for PlayerEntity {
    fn build(self, builder: T) -> Entity {
        builder
            .with(self.player)
            .with(self.class)
            .with(self.pos)
            .with(self.size)
            .with(self.vel)
            .with(self.acc)
            .with(self.lives)
            .with(self.asset)
            .with(self.ori)
            .build()
    }
}

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct BulletEntity {
    pub pos: Pos,
    pub size: Size,
    pub vel: Vel,
    pub acc: Acc,
    pub asset: Asset,
    pub ori: Ori,
    pub owner: Owner,
    pub class: Class,
    pub damage: Damage,
}

impl<T: Builder> BuildEntity<T> for BulletEntity {
    fn build(self, builder: T) -> Entity {
        builder
            .with(self.pos)
            .with(self.size)
            .with(self.vel)
            .with(self.acc)
            .with(self.asset)
            .with(self.ori)
            .with(self.owner)
            .with(self.class)
            .with(self.damage)
            .with(Bullet)
            .build()
    }
}

pub trait CreateEntity<T: Builder, E: BuildEntity<T>>: Sized {
    fn builder(self) -> T;

    fn create(self, entity: E) -> Entity {
        entity.build(self.builder())
    }
}

impl<'a, E> CreateEntity<EntityBuilder<'a>, E> for EntityBuilder<'a>
where
    E: BuildEntity<EntityBuilder<'a>>,
{
    fn builder(self) -> Self {
        self
    }
}

impl<'a, E> CreateEntity<LazyBuilder<'a>, E> for LazyBuilder<'a>
where
    E: BuildEntity<LazyBuilder<'a>>,
{
    fn builder(self) -> Self {
        self
    }
}

use crate::{components::*, systems::Systems};
use derive_new::new;
use serde::{Deserialize, Serialize};
use shred_derive::SystemData;
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

#[derive(new, Debug)]
pub struct PlayerEntityRef<'a> {
    pub player: &'a Player,
    pub class: &'a Class,
    pub pos: &'a Pos,
    pub size: &'a Size,
    pub vel: &'a Vel,
    pub acc: &'a Acc,
    pub lives: &'a Lives,
    pub asset: &'a Asset,
    pub ori: &'a Ori,
}

impl<'a> PlayerEntityRef<'a> {
    pub fn from(t: PlayerEntityRefTuple<'a>) -> Self {
        Self::new(t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8)
    }

    pub fn to(self) -> PlayerEntityRefTuple<'a> {
        (
            self.player,
            self.class,
            self.pos,
            self.size,
            self.vel,
            self.acc,
            self.lives,
            self.asset,
            self.ori,
        )
    }
}

#[derive(new, Debug)]
pub struct PlayerEntityMutRef<'a> {
    pub player: &'a mut Player,
    pub class: &'a mut Class,
    pub pos: &'a mut Pos,
    pub size: &'a mut Size,
    pub vel: &'a mut Vel,
    pub acc: &'a mut Acc,
    pub lives: &'a mut Lives,
    pub asset: &'a mut Asset,
    pub ori: &'a mut Ori,
}

impl<'a> PlayerEntityMutRef<'a> {
    pub fn from(t: PlayerEntityMutRefTuple<'a>) -> Self {
        Self::new(t.0, t.1, t.2, t.3, t.4, t.5, t.6, t.7, t.8)
    }

    pub fn to(self) -> PlayerEntityMutRefTuple<'a> {
        (
            self.player,
            self.class,
            self.pos,
            self.size,
            self.vel,
            self.acc,
            self.lives,
            self.asset,
            self.ori,
        )
    }
}

pub type PlayerEntityRefTuple<'a> = (
    &'a Player,
    &'a Class,
    &'a Pos,
    &'a Size,
    &'a Vel,
    &'a Acc,
    &'a Lives,
    &'a Asset,
    &'a Ori,
);

pub type PlayerEntityMutRefTuple<'a> = (
    &'a mut Player,
    &'a mut Class,
    &'a mut Pos,
    &'a mut Size,
    &'a mut Vel,
    &'a mut Acc,
    &'a mut Lives,
    &'a mut Asset,
    &'a mut Ori,
);

pub type PlayerEntityRead<'a> = (
    ReadStorage<'a, Player>,
    ReadStorage<'a, Class>,
    ReadStorage<'a, Pos>,
    ReadStorage<'a, Size>,
    ReadStorage<'a, Vel>,
    ReadStorage<'a, Acc>,
    ReadStorage<'a, Lives>,
    ReadStorage<'a, Asset>,
    ReadStorage<'a, Ori>,
);

pub type PlayerEntityWrite<'a, 'b> = (
    &'b mut WriteStorage<'a, Player>,
    &'b mut WriteStorage<'a, Class>,
    &'b mut WriteStorage<'a, Pos>,
    &'b mut WriteStorage<'a, Size>,
    &'b mut WriteStorage<'a, Vel>,
    &'b mut WriteStorage<'a, Acc>,
    &'b mut WriteStorage<'a, Lives>,
    &'b mut WriteStorage<'a, Asset>,
    &'b mut WriteStorage<'a, Ori>,
);

#[derive(SystemData)]
pub struct PlayerEntityReadData<'a> {
    pub player: ReadStorage<'a, Player>,
    pub cls: ReadStorage<'a, Class>,
    pub pos: ReadStorage<'a, Pos>,
    pub siz: ReadStorage<'a, Size>,
    pub vel: ReadStorage<'a, Vel>,
    pub acc: ReadStorage<'a, Acc>,
    pub lives: ReadStorage<'a, Lives>,
    pub asset: ReadStorage<'a, Asset>,
    pub ori: ReadStorage<'a, Ori>,
}

#[derive(SystemData)]
pub struct PlayerEntityWriteData<'a> {
    pub player: WriteStorage<'a, Player>,
    pub cls: WriteStorage<'a, Class>,
    pub pos: WriteStorage<'a, Pos>,
    pub size: WriteStorage<'a, Size>,
    pub vel: WriteStorage<'a, Vel>,
    pub acc: WriteStorage<'a, Acc>,
    pub lives: WriteStorage<'a, Lives>,
    pub asset: WriteStorage<'a, Asset>,
    pub ori: WriteStorage<'a, Ori>,
}

impl<'a> PlayerEntityWriteData<'a> {
    pub fn filter<'b>(&'b mut self) -> PlayerEntityWrite<'a, 'b> {
        (
            &mut self.player,
            &mut self.cls,
            &mut self.pos,
            &mut self.size,
            &mut self.vel,
            &mut self.acc,
            &mut self.lives,
            &mut self.asset,
            &mut self.ori,
        )
    }
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

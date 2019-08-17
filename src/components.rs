use crate::vector::Vector;
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;
use uuid::Uuid;

#[derive(new, PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Class(pub u64);

impl Class {
    pub fn neutral() -> Self {
        Self(0)
    }

    pub fn chiba() -> Self {
        Self(1)
    }

    pub fn saitama() -> Self {
        Self(2)
    }
}

impl_vector!(Pos);
impl_vector!(Vel);
impl_vector!(Acc);
impl_vector!(Ori);
impl_vector!(Size);

impl Ori {
    pub fn left() -> Self {
        Self::new(-1.0, 0.0)
    }

    pub fn right() -> Self {
        Self::new(1.0, 0.0)
    }

    pub fn up() -> Self {
        Self::new(0.0, 1.0)
    }

    pub fn down() -> Self {
        Self::new(0.0, -1.0)
    }
}

impl Acc {
    pub fn gravity() -> Self {
        Self::new(0.0, -0.15)
    }
}

#[derive(new, Component, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct AssetId(pub u64);

#[derive(new, Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: Uuid,
    pub seqno: u64,
    pub cls: Class,
    pub lives: u64,
}

impl Player {
    /// Spawn a new player
    pub fn spawn(lives: u64, cls: Class) -> Self {
        Self::new(Uuid::new_v4(), 0, cls, lives)
    }

    /// Spawn a bullet from the player
    pub fn bullet(&mut self, dmg: u64) -> Bullet {
        self.seqno += 1;
        Bullet::new(self.id, self.seqno, self.cls, dmg)
    }
}

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct Bullet {
    pub pid: Uuid,
    pub seqno: u64,
    pub cls: Class,
    pub dmg: u64,
}

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct Landmark {
    pub id: Uuid,
    pub cls: Class,
    pub lives: u64,
}

impl Landmark {
    /// Spawn a new landmark
    pub fn spawn(lives: u64) -> Self {
        Self::new(Uuid::new_v4(), Class::neutral(), lives)
    }

    /// Concurr the landmark
    pub fn concurred(&mut self, ply: &Player, lives: u64) {
        self.cls = ply.cls;
        self.lives = lives;
    }
}

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct Block;

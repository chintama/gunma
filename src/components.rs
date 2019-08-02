use crate::vector::Vector;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Class(pub u64);

pub const CLASS_NEUTRAL: Class = Class(0);
pub const CLASS_CHIBA: Class = Class(1);
pub const CLASS_SAITAMA: Class = Class(2);

impl_vector!(Pos);
impl_vector!(Vel);
impl_vector!(Size);
impl_vector!(Acc);

impl Acc {
    pub fn gravity() -> Self {
        Self::new(0.0, -0.15)
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u64,
    pub class: Class,
    pub lives: u64,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct User;

impl Player {
    pub fn new(id: u64, class: Class, lives: u64) -> Self {
        Self { id, class, lives }
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Bullet {
    pub id: u64,
    pub class: Class,
}

impl Bullet {
    pub fn new(id: u64, class: Class) -> Self {
        Self { id, class }
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Landmark {
    pub lives: u64,
    pub class: Class,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Background;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Block;

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Dir(pub f32);

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Asset(pub u64);

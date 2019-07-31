use crate::vector::Vector;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Class(pub u64);

pub const CLASS_NEUTRAL: Class = Class(0);
pub const CLASS_CHIBA: Class = Class(1);
pub const CLASS_SAITAMA: Class = Class(2);

impl_vector!(Pos);
impl_vector!(Vel);
impl_vector!(Size);
impl_vector!(Acc);

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub id: u64,
    pub cls: Class,
    pub lives: u64,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Enemy;

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Gun {
    pub bullets: u64,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Bullet {
    pub class: Class,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Landmark {
    pub lives: u64,
    pub class: Class,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Block;

#[derive(Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Dir(pub f32);

#[derive(Component, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Asset(pub u64);

use crate::{resources::ObjectId, vector::Vector};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

#[derive(new, PartialEq, Eq, Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Class(pub u64);

pub const CLASS_NEUTRAL: Class = Class(0);
pub const CLASS_CHIBA: Class = Class(1);
pub const CLASS_SAITAMA: Class = Class(2);

#[derive(new, Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Player(pub u64);

#[derive(new, Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Owner(pub u64);

impl_vector!(Pos);
impl_vector!(Vel);
impl_vector!(Size);
impl_vector!(Acc);
impl_vector!(Ori);

impl Acc {
    pub fn gravity() -> Self {
        Self::new(0.0, -0.15)
    }
}

#[derive(new, Component, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Lives(pub u64);

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct Bullet;

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct Damage(pub u64);

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct Landmark;

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct Background;

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct Block;

#[derive(new, Component, Clone, Debug, Serialize, Deserialize)]
pub struct User;

#[derive(new, Component, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Asset(pub u64);

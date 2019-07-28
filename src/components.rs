use quicksilver::geom::{Rectangle, Vector};
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;
use std::cell::Cell;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Class(pub u64);

pub const CLASS_NEUTRAL: Class = Class(0);
pub const CLASS_CHIBA: Class = Class(1);
pub const CLASS_SAITAMA: Class = Class(2);

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Pos(pub Vector);

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector { x, y })
    }

    pub fn zero() -> Self {
        Self(Vector::ZERO)
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Vel(pub Vector);

impl Vel {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector { x, y })
    }

    pub fn zero() -> Self {
        Self(Vector::ZERO)
    }

    pub fn xcomp(&self) -> Self {
        if self.0.x == 0.0 {
            Self::zero()
        } else {
            Self::new(self.0.x / self.0.x.abs(), 0.0)
        }
    }

    pub fn ycomp(&self) -> Self {
        if self.0.y == 0.0 {
            Self::zero()
        } else {
            Self::new(0.0, self.0.y / self.0.y.abs())
        }
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Size(pub Vector);

impl Size {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector { x, y })
    }

    pub fn zero() -> Self {
        Self(Vector::ZERO)
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Acc(pub Vector);

impl Acc {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector { x, y })
    }

    pub fn zero() -> Self {
        Self(Vector::ZERO)
    }
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Player {
    pub lives: u64,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Enemy {
    pub lives: u64,
    pub class: Class,
}

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

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Dir(pub f32);

#[derive(Component, Clone, Debug, Serialize, Deserialize)]
pub struct Asset(pub u64);

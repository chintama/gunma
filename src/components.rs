use quicksilver::geom::Vector;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use specs_derive::Component;

#[derive(Debug, Serialize, Deserialize)]
pub struct Class(pub u64);

pub const CLASS_NEUTRAL: Class = Class(0);
pub const CLASS_CHIBA: Class = Class(1);
pub const CLASS_SAITAMA: Class = Class(2);

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Pos(pub Vector);

impl Pos {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector { x, y })
    }

    pub fn zero() -> Self {
        Self(Vector::ZERO)
    }
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Vel(pub Vector);

impl Vel {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector { x, y })
    }

    pub fn zero() -> Self {
        Self(Vector::ZERO)
    }
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Acc(pub Vector);

impl Acc {
    pub fn new(x: f32, y: f32) -> Self {
        Self(Vector { x, y })
    }

    pub fn zero() -> Self {
        Self(Vector::ZERO)
    }
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Player {
    pub lives: u64,
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Enemy {
    pub lives: u64,
    pub class: Class,
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Gun {
    pub bullets: u64,
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Bullets {
    pub class: Class,
}

#[derive(Component, Debug, Serialize, Deserialize)]
pub struct Landmark {
    pub lives: u64,
    pub class: Class,
}

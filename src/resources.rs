use crate::components::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Collision,
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Events(pub Vec<Event>);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PlayerUpdate {
    pub pos: Pos,
    pub player: Player,
    pub dir: Dir,
    pub vel: Vel,
    pub acc: Acc,
}

impl PlayerUpdate {
    pub fn new(pos: Pos, player: Player, dir: Dir, vel: Vel, acc: Acc) -> Self {
        Self {
            pos,
            player,
            dir,
            vel,
            acc,
        }
    }
}

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerUpdates(pub HashMap<u64, PlayerUpdate>);

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct Action {
    pub jump: bool,
    pub left: bool,
    pub right: bool,
    pub take: bool,
    pub drop: bool,
    pub update: bool,
}

impl Action {
    pub fn jump(&mut self) {
        self.jump = true;
        self.update = true;
    }

    pub fn left(&mut self) {
        self.left = true;
        self.update = true;
    }

    pub fn right(&mut self) {
        self.right = true;
        self.update = true;
    }

    pub fn take(&mut self) {
        self.take = true;
        self.update = true;
    }

    pub fn drop(&mut self) {
        self.drop = true;
        self.update = true;
    }

    pub fn clear(&mut self) -> Self {
        std::mem::replace(self, Self::default())
    }
}

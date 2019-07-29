use crate::components::Player;
use serde::{Deserialize, Serialize};

#[derive(Default, Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Option<u64>,
}

impl User {
    pub fn set(&mut self, player: &Player) {
        self.id = Some(player.id);
    }

    pub fn is_me(&self, player: &Player) -> bool {
        self.id.is_some() && self.id.unwrap() == player.id
    }
}

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

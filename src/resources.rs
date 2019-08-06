use crate::{components::*, events::*};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs::prelude::*;

///
/// Resource to indicate which entity the client user is using
///
#[derive(new, Default, Clone, Debug)]
pub struct User {
    pub id: Option<u64>,
    pub entity: Option<Entity>,
}

///
/// Resource to deliver events
///
#[derive(new, Default, Clone, Debug)]
pub struct Events(pub Vec<Event>);

///
/// Resource to handle user inputs
///
#[derive(new, Default, Clone, Debug, Serialize, Deserialize)]
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

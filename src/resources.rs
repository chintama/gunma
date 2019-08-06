use crate::{components::*, events::*, io::Io};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex, MutexGuard},
};

///
/// Resource to indicate which entity the client user is using
///
#[derive(new, Default, Clone, Debug)]
pub struct UserEntity {
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

///
/// Resource to communicate each other
///
#[derive(new, Default, Clone)]
pub struct Network(pub Option<Arc<Mutex<Io>>>);

impl Network {
    pub fn set(&mut self, io: Io) {
        self.0 = Some(Arc::new(Mutex::new(io)));
    }

    pub fn get<'a>(&'a self) -> MutexGuard<'a, Io> {
        self.0.as_ref().unwrap().lock().unwrap()
    }
}

///
/// System-wide global unique id assigned by server side
///
#[derive(new, PartialEq, Eq, Hash, Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ObjectId(pub u64);

///
/// Mapping between object ids (system-wide) and entities (client-wide)
///
#[derive(new, Default, Clone)]
pub struct Objects(pub HashMap<ObjectId, Entity>);

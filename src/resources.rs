use crate::{components::*, events::*};
use derive_new::new;
use serde::{Deserialize, Serialize};
use specs::prelude::*;
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, Mutex, MutexGuard},
};

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
#[derive(new, Clone)]
pub struct Res<T>(pub Option<Arc<Mutex<T>>>);

impl<T> std::default::Default for Res<T> {
    fn default() -> Self {
        Self(None)
    }
}

impl<T> Res<T> {
    pub fn set(&mut self, inner: T) {
        self.0 = Some(Arc::new(Mutex::new(inner)));
    }

    pub fn get<'a>(&'a self) -> MutexGuard<'a, T> {
        self.0.as_ref().unwrap().lock().unwrap()
    }
}

pub type UserEntity = Res<(u64, Entity)>;
pub type ClientQueue = Res<wsq::ClientQueue<Event>>;
pub type ServerQueue = Res<wsq::ServerQueue<Event>>;

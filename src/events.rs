use crate::{entities::*, resources::Action};
use derive_new::new;
use serde::{Deserialize, Serialize};

///
/// Events
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    Collision,
    PlayerState(PlayerState),
}

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub state: PlayerEntity,
    pub action: Option<Action>,
}

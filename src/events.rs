use crate::{components::*, entities::*, resources::Action};
use derive_new::new;
use serde::{Deserialize, Serialize};

///
/// Login request
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    pub cls: Class,
}

///
/// Login acknowledge
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct LoginAck {
    pub id: u64,
    pub pos: Pos,
    pub ori: Ori,
    pub lives: Lives,
}

///
/// User input
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    pub id: u64,
    pub seqno: u64,
    pub action: Action,
}

///
/// Player state
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    pub id: u64,
    pub seqno: u64,
    pub pos: Pos,
    pub vel: Vel,
    pub acc: Acc,
    pub ori: Ori,
}

///
/// World state
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub players: Vec<PlayerState>,
}

///
/// Event data
///
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Event {
    /// Login (c2s)
    Login(Login),
    /// Login ack (s2c)
    LoginAck(LoginAck),
    /// User input (c2s)
    Input(Input),
    /// Authorative state (s2c)
    State(State),
}

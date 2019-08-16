use crate::{components::*, resources::Action};
use derive_new::new;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

///
/// Login request
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    /// Class requested by a user
    pub cls: Class,
}

///
/// Login acknowledge
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct LoginAck {
    /// Player initial state
    pub ps: PlayerState,
}

///
/// User input
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct Input {
    /// Uuid of the user
    pub id: Uuid,
    /// Sequence number of the input
    pub seqno: u64,
    /// Content of the input
    pub act: Action,
}

///
/// Player state
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct PlayerState {
    /// Sequence number of the event
    pub seqno: u64,
    pub pos: Pos,
    pub vel: Vel,
    pub acc: Acc,
    pub ori: Ori,
    pub siz: Size,
    pub aid: AssetId,
    pub ply: Player,
}

///
/// World state
///
#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct State {
    pub plys: Vec<PlayerState>,
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

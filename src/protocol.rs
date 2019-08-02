use crate::{components::*, resources::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GetTerrain {
    pub pos: Pos,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub id: u64,
    pub pos: Pos,
    pub size: Size,
    pub asset: Asset,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SendAction {
    pub player: Player,
    pub pos: Pos,
    pub vel: Vel,
    pub acc: Acc,
    pub dir: Dir,
    pub action: Action,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    pub cls: Class,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LoginAck {
    pub player: Player,
    pub spawn: Pos,
}

impl LoginAck {
    pub fn new(player: Player, spawn: Pos) -> Self {
        Self { player, spawn }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    Login(Login),
    LoginAck(LoginAck),
    GetTerrain(GetTerrain),
    GetAllTerrain,
    Terrain(Terrain),
    EndTerrain,
    SendAction(SendAction),
}

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
    pub id: u64,
    pub action: Action,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum Message {
    GetTerrain(GetTerrain),
    GetAllTerrain,
    Terrain(Terrain),
    EndTerrain,
    SendAction(SendAction),
}

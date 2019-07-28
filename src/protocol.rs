use crate::{components::*, entities::*};
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
pub enum Message {
    GetTerrain(GetTerrain),
    Terrain(Terrain),
}

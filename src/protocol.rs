use crate::{components::*, entities::*, resources::*};
use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct GetTerrain {
    pub pos: Pos,
}

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct Terrain {
    pub entity: TerrainEntity,
}

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct SendAction {
    pub player: PlayerEntity,
    pub action: Action,
}

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct Login {
    pub cls: Class,
}

#[derive(new, Clone, Debug, Serialize, Deserialize)]
pub struct LoginAck {
    pub entity: PlayerEntity,
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

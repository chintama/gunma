use crate::{components::*, entities::*};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct GetTerrain {
    pos: Pos,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Terrain {
    id: u64,
    pos: Pos,
    size: Size,
    asset: Asset,
}

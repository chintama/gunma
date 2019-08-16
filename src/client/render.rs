use crate::components::AssetId;
use log::*;
use quicksilver::{graphics::Image, prelude::*};
use std::collections::HashMap;

pub type AssetsMap = HashMap<AssetId, Image>;

pub fn load_image(s: &str) -> Image {
    Image::load(s).wait().unwrap()
}

pub fn load_assets() -> AssetsMap {
    let mut assets = HashMap::new();

    assets.insert(AssetId(1), load_image("ferris.png"));
    assets.insert(AssetId(2), load_image("ferris-f.png"));
    assets.insert(AssetId(3), load_image("cpp.png"));
    assets.insert(AssetId(4), load_image("bjarne.png"));

    assets.insert(AssetId(100), load_image("bubble.png"));
    assets.insert(AssetId(200), load_image("ground.png"));
    assets.insert(AssetId(900), load_image("gameover.png"));
    assets.insert(AssetId(901), load_image("beach.png"));

    assets
}

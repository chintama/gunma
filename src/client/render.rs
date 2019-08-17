use crate::components::AssetId;
use log::*;
use quicksilver::{geom::Rectangle, graphics::Image, prelude::*};
use std::collections::HashMap;

pub type AssetsMap = HashMap<AssetId, Image>;

pub fn load_image(s: &str) -> Image {
    info!("Loading {}", s);
    let img = Image::load(s).wait().unwrap();
    info!("Loaded {}", s);
    img
}

///
/// Ground assets
///
/// id = 1xxyy
///
/// where:
///   xx = x-coordinate in the sprite (0-4)
///   yy = y-coordinate in the sprite (0-4)
///
fn process_ground(img: &Image, assets: &mut AssetsMap) {
    let gnd = img.subimage(Rectangle::new((400.0, 400.0), (240.0, 240.0)));
    for y in 0..(240 / 60) {
        for x in 0..(240 / 60) {
            let aid = AssetId(10000 + x * 100 + y);
            let (x, y) = (x as f32, y as f32);
            let simg = gnd.subimage(Rectangle::new((x * 60.0, y * 60.0), (60.0, 60.0)));
            assets.insert(aid, simg);
        }
    }
}

///
/// Plate assets
///
/// id = 2xx00
///
/// where:
///   xx = x-coordinate in the sprite (0-4)
///
fn process_plate(img: &Image, assets: &mut AssetsMap) {
    let plt = img.subimage(Rectangle::new((640.0, 240.0), (240.0, 75.0)));

    for x in 0..(240 / 60) {
        let aid = AssetId(20000 + x * 100);
        let x = x as f32;
        let simg = plt.subimage(Rectangle::new((x * 60.0, 0.0), (60.0, 75.0)));
        assets.insert(aid, simg);
    }
}

///
/// Tree asset
///
/// id = 30000
///
fn process_tree(img: &Image, assets: &mut AssetsMap) {
    let tree = img.subimage(Rectangle::new((970.0, 405.0), (220.0, 315.0)));
    assets.insert(AssetId(30000), tree);
}

///
/// Grass assets
///
/// id = 400tt
///
/// where:
///   tt = assets type
///
///     0: grass 1
///     1: grass 2
///     2: grass 3
///     3: grass 4
///     4: with flower 1
///     5: with flower 2
///
fn process_grass(img: &Image, assets: &mut AssetsMap) {
    let g = img.subimage(Rectangle::new((720.0, 690.0), (60.0, 30.0)));
    assets.insert(AssetId(40000), g);
    let g = img.subimage(Rectangle::new((880.0, 695.0), (65.0, 25.0)));
    assets.insert(AssetId(40001), g);
    let g = img.subimage(Rectangle::new((880.0, 765.0), (60.0, 35.0)));
    assets.insert(AssetId(40002), g);
    let g = img.subimage(Rectangle::new((885.0, 845.0), (75.0, 35.0)));
    assets.insert(AssetId(40003), g);

    let g = img.subimage(Rectangle::new((720.0, 770.0), (80.0, 30.0)));
    assets.insert(AssetId(40004), g);
    let g = img.subimage(Rectangle::new((720.0, 885.0), (60.0, 25.0)));
    assets.insert(AssetId(40005), g);
}

///
/// Bridge assets
///
/// id = 50uxx
///
/// where:
///
///    u: upper level if 1 (0 or 1)
///    xx: x-cordinate (0-2)
///
fn process_bridge(img: &Image, assets: &mut AssetsMap) {
    let bdg = img.subimage(Rectangle::new((0.0, 680.0), (160.0, 120.0)));

    // Upper 1,2,3
    let b = bdg.subimage(Rectangle::new((55.0, 0.0), (60.0, 40.0)));
    assets.insert(AssetId(50100), b);
    let b = bdg.subimage(Rectangle::new((65.0, 0.0), (60.0, 40.0)));
    assets.insert(AssetId(50101), b);
    let b = bdg.subimage(Rectangle::new((80.0, 0.0), (60.0, 40.0)));
    assets.insert(AssetId(50102), b);

    // Lower 1,2,3
    let b = bdg.subimage(Rectangle::new((80.0, 40.0), (60.0, 20.0)));
    assets.insert(AssetId(50000), b);
}

///
/// Ledge assets
///
/// id = 6000d
///
/// where:
///
///   d: left (0) or right (1)
///
fn process_ledge(img: &Image, assets: &mut AssetsMap) {
    let ldg = img.subimage(Rectangle::new((720.0, 480.0), (160.0, 80.0)));

    let b = ldg.subimage(Rectangle::new((20.0, 0.0), (60.0, 60.0)));
    assets.insert(AssetId(60000), b);
    let b = ldg.subimage(Rectangle::new((80.0, 0.0), (60.0, 60.0)));
    assets.insert(AssetId(60001), b);
}

fn process_tilesets(assets: &mut AssetsMap) {
    let img = load_image("tileset.png");

    process_ground(&img, assets);
    process_plate(&img, assets);
    process_tree(&img, assets);
    process_grass(&img, assets);
    process_bridge(&img, assets);
    process_ledge(&img, assets);
}

pub fn load_assets() -> AssetsMap {
    let mut assets = HashMap::new();

    process_tilesets(&mut assets);

    assets.insert(AssetId(1), load_image("ferris.png"));
    assets.insert(AssetId(2), load_image("ferris-f.png"));

    assets.insert(AssetId(100), load_image("bubble.png"));

    assets.insert(AssetId(901), load_image("skybg.png"));
    assets.insert(AssetId(902), load_image("water.png"));
    assets.insert(AssetId(903), load_image("water-reflex.png"));
    assets.insert(AssetId(904), load_image("clouds.png"));

    assets
}

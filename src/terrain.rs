use crate::components::{AssetId, Pos, Size};
use derive_new::new;
use log::*;
use rand::Rng;
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

#[derive(new, Serialize, Deserialize)]
pub struct Terrain {
    pub pos: Pos,
    pub siz: Size,
    pub aid: AssetId,
    pub blk: bool,
}

impl Terrain {
    fn blk(pos: Pos, siz: Size, aid: AssetId) -> Self {
        Self::new(pos, siz, aid, true)
    }

    fn bg(pos: Pos, siz: Size, aid: AssetId) -> Self {
        Self::new(pos, siz, aid, false)
    }
}

trait Type {
    fn is_block(&self) -> bool;

    fn is_tree(&self) -> bool;

    fn is_space(&self) -> bool;

    fn is_bridge(&self) -> bool;
}

impl Type for u8 {
    fn is_block(&self) -> bool {
        *self == b'+' || *self == b'-' || *self == b'|'
    }

    fn is_tree(&self) -> bool {
        *self == b'T'
    }

    fn is_space(&self) -> bool {
        *self == b' '
    }

    fn is_bridge(&self) -> bool {
        *self == b'='
    }
}

pub fn parse_terrain(path: &str) -> Vec<Terrain> {
    info!("Parsing terrain data from file: {}", path);

    let f = File::open(path).unwrap();
    let f = BufReader::new(f);
    let d: Vec<_> = f.lines().map(|l| l.unwrap().as_bytes().to_vec()).collect();
    parse_tdata(d)
}

pub fn read_terrain(path: &str) -> Vec<Terrain> {
    let f = File::open(path).unwrap();
    serde_json::from_reader(f).unwrap()
}

pub fn write_terrain(path: &str, t: Vec<Terrain>) {
    let f = File::create(path).unwrap();
    serde_json::to_writer(f, &t).unwrap();
}

pub fn parse_tdata(data: Vec<Vec<u8>>) -> Vec<Terrain> {
    let mut rng = rand::thread_rng();

    let topos = |x, y| {
        let h = data.len();
        Pos::new(x as f32 * 60.0, (h - y) as f32 * 60.0 - 120.0)
    };
    let chk = |x, y| {
        if x < 0 {
            b' '
        } else if y < 0 {
            b' '
        } else {
            *data
                .get(y)
                .and_then(|v: &Vec<u8>| v.get(x))
                .unwrap_or(&b' ')
        }
    };
    let u = |x, y| chk(x, y - 1);
    let d = |x, y| chk(x, y + 1);
    let r = |x, y| chk(x + 1, y);
    let l = |x, y| chk(x - 1, y);

    let mut ts = Vec::new();

    for (x, y, p) in data
        .iter()
        .enumerate()
        .map(|(y, v)| v.iter().enumerate().map(move |(x, p)| (x, y, p)))
        .flatten()
    {
        if p.is_block() {
            let t = if !u(x, y).is_block() && !d(x, y).is_block() {
                20000
            } else {
                10000
            };

            let xx = if !l(x, y).is_block() {
                0
            } else if !r(x, y).is_block() {
                3
            } else {
                1
            };

            let yy = if !u(x, y).is_block() {
                0
            } else if !d(x, y).is_block() {
                3
            } else {
                1
            };

            let mid = xx == 1 && yy == 1;
            let aid = if mid && chk(x + 1, y - 1).is_space() {
                AssetId::new(60000)
            } else if mid && chk(x - 1, y - 1).is_space() {
                AssetId::new(60001)
            } else {
                AssetId::new(t + xx * 100 + yy)
            };

            ts.push(Terrain::blk(topos(x, y), Size::new(60.0, 60.0), aid));

            let br = r(x, y).is_bridge();
            let bl = l(x, y).is_bridge();

            if br || bl {
                // upper
                let aid = if br {
                    AssetId::new(50102)
                } else {
                    AssetId::new(50100)
                };
                ts.push(Terrain::bg(topos(x, y - 1), Size::new(60.0, 60.0), aid));

                // lower
                let aid = AssetId::new(50000);
                let mut p = topos(x, y);
                p.y += 40.0;
                ts.push(Terrain::blk(p, Size::new(60.0, 20.0), aid));
            } else {
                // generate grass sometimes
                if yy == 0 {
                    let gid = rng.gen::<u64>() % 10u64;

                    if gid <= 5 {
                        let aid = AssetId::new(40000 + gid);
                        ts.push(Terrain::bg(topos(x, y - 1), Size::new(60.0, 30.0), aid));
                    }
                }
            }
        } else if p.is_bridge() {
            let aid = AssetId::new(50101);
            ts.push(Terrain::bg(topos(x, y - 1), Size::new(60.0, 60.0), aid));
            let aid = AssetId::new(50000);
            let mut p = topos(x, y);
            p.y += 40.0;
            ts.push(Terrain::blk(p, Size::new(60.0, 20.0), aid));
        } else if p.is_tree() {
            let aid = AssetId::new(30000);
            ts.push(Terrain::bg(topos(x - 1, y), Size::new(240.0, 320.0), aid));
        }
    }

    ts
}

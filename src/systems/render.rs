use crate::{
    client::render::AssetsMap,
    components::{AssetId, Block, Bullet, Ori, Player, Pos, Size},
    resources::*,
};
use quicksilver::{
    geom::{Rectangle, Vector},
    graphics::{Background::Col, Color},
    lifecycle::Window,
    prelude::*,
};
use specs::prelude::*;

pub struct Render<'a> {
    window: &'a mut Window,
    assets: AssetsMap,
}

impl<'a> Render<'a> {
    pub fn new(window: &'a mut Window, assets: AssetsMap) -> Self {
        Self { window, assets }
    }

    fn img(&self, id: AssetId) -> Option<Image> {
        self.assets.get(&id).map(|img| img.clone())
    }

    fn drw(&mut self, pos: Pos, siz: Size, origin: Pos, center: Pos, bg: Background) {
        let size = self.window.screen_size();
        let pos = (Pos::new(pos.x, size.y - pos.y - siz.y) - origin) + center;
        let pos = Vector::new(pos.x, pos.y);
        let siz = Vector::new(siz.x, siz.y);
        let rect = Rectangle::new(pos, siz);
        let inv = Transform::IDENTITY;
        self.window.draw_ex(&rect, bg, inv, 1.0);
    }
}

impl<'a, 'b> System<'a> for Render<'b> {
    type SystemData = (
        Entities<'a>,
        Read<'a, UserEntity>,
        ReadStorage<'a, Pos>,
        ReadStorage<'a, Ori>,
        ReadStorage<'a, AssetId>,
        ReadStorage<'a, Size>,
        ReadStorage<'a, Bullet>,
        ReadStorage<'a, Block>,
        ReadStorage<'a, Player>,
    );

    fn run(&mut self, (e, user, pos, ori, aid, siz, blt, blk, ply): Self::SystemData) {
        let ep = user.get().1;

        let center = self.window.screen_size() / 2.0;
        let center = Pos::new(center.x, 0.0);

        let origin = {
            let pos = pos.get(ep).unwrap();
            Pos::new(pos.x, 0.0)
        };

        self.window.clear(Color::WHITE).unwrap();
        let bgimg = self.img(AssetId(901)).unwrap();
        self.window.draw(
            &Rectangle::new(Vector::new(0.0, 0.0), Vector::new(800.0, 600.0)),
            Blended(&bgimg, Color::from_rgba(250, 250, 250, 0.5)),
        );

        let mut drw = |e, pos: &Pos, siz: &Size| {
            let col = if ply.get(e).is_some() {
                Col(Color::GREEN)
            } else if blt.get(e).is_some() {
                Col(Color::BLACK)
            } else if blk.get(e).is_some() {
                Col(Color::BLUE)
            } else {
                Col(Color::RED)
            };

            let img = match aid.get(e) {
                Some(id) => {
                    let id = if ply.get(e).is_some()
                        && ori.get(e).unwrap_or(&Ori::new(1.0, 0.0)).x > 0.0
                    {
                        AssetId(2)
                    } else {
                        *id
                    };
                    self.img(id)
                }
                None => None,
            };

            match img {
                Some(img) => {
                    self.drw(*pos, *siz, origin, center, Img(&img));
                }
                None => {
                    self.drw(*pos, *siz, origin, center, col);
                }
            }
        };

        for (e, pos, siz) in (&e, &pos, &siz).join() {
            if blt.get(e).is_some() {
                continue;
            }
            drw(e, pos, siz)
        }

        for (e, pos, siz, _) in (&e, &pos, &siz, &blt).join() {
            drw(e, pos, siz)
        }
    }
}

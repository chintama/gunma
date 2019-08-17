use crate::components::*;
use log::*;

use ncollide2d::{
    math::Isometry,
    query::{contact, time_of_impact},
    shape::Cuboid,
};

pub fn toi(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size, v2: &Vel) -> f32 {
    let m1 = *p1 + *s1 / 2.0;
    let m1 = Isometry::translation(m1.x, m1.y);
    let c1 = Cuboid::new((*s1 / 2.0).to_vec());
    let v1 = v1.to_vec();

    let m2 = *p2 + *s2 / 2.0;
    let m2 = Isometry::translation(m2.x, m2.y);
    let c2 = Cuboid::new((*s2 / 2.0).to_vec());
    let v2 = v2.to_vec();

    time_of_impact(&m1, &v1, &c1, &m2, &v2, &c2)
        .unwrap_or(1.0)
        .min(1.0)
}

pub fn normal(p1: &Pos, s1: &Size, p2: &Pos, s2: &Size) -> Option<Vel> {
    let m1 = *p1 + *s1 / 2.0;
    let m1 = Isometry::translation(m1.x, m1.y);
    let c1 = Cuboid::new((*s1 / 2.0).to_vec());

    let m2 = *p2 + *s2 / 2.0;
    let m2 = Isometry::translation(m2.x, m2.y);
    let c2 = Cuboid::new((*s2 / 2.0).to_vec());

    contact(&m1, &c1, &m2, &c2, 3.0).map(|c| {
        let x = c.normal.as_ref()[0].round();
        let y = c.normal.as_ref()[1].round();
        Vel::new(x, y)
    })
}

pub fn cease_vel(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size) -> (Option<Vel>, Vel) {
    let res = match normal(p1, s1, p2, s2) {
        Some(n) => {
            let mut v = v1.clone();

            if n.x * v1.x > 0.0 {
                v.x = 0.0;
            }
            if n.y * v1.y > 0.0 {
                v.y = 0.0;
            }

            (Some(n), v)
        }
        None => (None, Vel::zero()),
    };

    res
}

pub fn update_vel(
    p1: &Pos,
    s1: &Size,
    v1: &Vel,
    p2: &Pos,
    s2: &Size,
    v2: &Vel,
) -> ((Option<Vel>, Vel), (Option<Vel>, Vel)) {
    let toi = toi(p1, s1, v1, p2, s2, v2);

    if toi == 0.0 {
        (cease_vel(p1, s1, v1, p2, s2), cease_vel(p2, s2, v2, p1, s1))
    } else {
        ((None, *v1 * toi), (None, *v2 * toi))
    }
}

pub fn collide(p1: &Pos, s1: &Size, v1: &Vel, p2: &Pos, s2: &Size, v2: &Vel) -> bool {
    toi(p1, s1, v1, p2, s2, v2) < 1.0
}

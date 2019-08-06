use crate::components::*;
use log::*;
use specs::prelude::*;

pub struct Print;

impl<'a> System<'a> for Print {
    type SystemData = (Entities<'a>, ReadStorage<'a, Pos>);

    fn run(&mut self, (e, pos): Self::SystemData) {
        for (e, pos) in (&e, &pos).join() {
            trace!("{:?}: pos={:?}", e, pos);
        }
    }
}

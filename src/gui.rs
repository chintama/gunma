use crate::systems::Systems;

use quicksilver::{
    geom::{Circle, Line, Rectangle, Transform, Triangle, Vector},
    graphics::{Background::Col, Color},
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};
use std::collections::BTreeMap;

struct DrawGeometry<'a, 'b> {
    sys: Systems<'a, 'b>,
}

impl<'a, 'b> State for DrawGeometry<'a, 'b> {
    fn new(sys: Systems<'a, 'b>) -> Result<DrawGeometry<'a, 'b>> {
        Ok(DrawGeometry { sys })
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                // Update sys.world()
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                // Update sys.world()
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }
        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        window.clear(Color::WHITE)?;
        window.draw(&Rectangle::new((400, 300), (50, 50)), Col(Color::RED));
        Ok(())
    }
}

pub fn run_chintama() {
    run::<DrawGeometry>("Draw Geometry", Vector::new(800, 600), Settings::default());
}

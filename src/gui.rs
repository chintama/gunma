use crate::{config::Config, systems::Systems};

use quicksilver::{
    geom::Vector,
    input::{ButtonState, Key},
    lifecycle::{run, Event, Settings, State, Window},
    Result,
};

struct Screen {
    sys: Systems,
}

impl State for Screen {
    fn new() -> Result<Screen> {
        Ok(Screen {
            sys: Systems::new(Config::default()).unwrap(),
        })
    }

    fn update(&mut self, _: &mut Window) -> Result<()> {
        self.sys.update();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        self.sys.fetch_action(|action| match *event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                action.left = true;
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                action.right = true;
            }
            Event::Key(Key::Up, ButtonState::Pressed) => {
                action.jump = true;
            }
            Event::Key(Key::Z, ButtonState::Pressed) => {
                action.take = true;
            }
            Event::Key(Key::X, ButtonState::Pressed) => {
                action.drop = true;
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        });

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        self.sys.render(window);
        Ok(())
    }
}

pub fn run_gui() {
    run::<Screen>("Chintama", Vector::new(800, 600), Settings::default());
}

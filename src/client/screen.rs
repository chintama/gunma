use crate::{
    client::{
        config::Config,
        render::{load_assets, AssetsMap},
    },
    components::*,
    resources::Action,
    systems::{render::Render, Systems},
};
use econf::load;
use quicksilver::{
    geom::Vector,
    input::{ButtonState, Key},
    lifecycle::{self, Event, Settings, State, Window},
    Result,
};

struct Screen {
    sys: Systems,
    action: Action,
    assets: AssetsMap,
}

impl State for Screen {
    fn new() -> Result<Screen> {
        let cfg = Config::default();
        let cfg = load(cfg, "CHINCLI");
        let mut sys = Systems::new().unwrap();

        sys.client_login(Class(cfg.class));

        Ok(Screen {
            action: Action::default(),
            sys,
            assets: load_assets(),
        })
    }

    fn update(&mut self, _: &mut Window) -> Result<()> {
        self.sys.client_update(Some(self.action.clone()));
        self.action.clear();
        Ok(())
    }

    fn event(&mut self, event: &Event, window: &mut Window) -> Result<()> {
        match *event {
            Event::Key(Key::Left, ButtonState::Pressed) => {
                self.action.left();
            }
            Event::Key(Key::Right, ButtonState::Pressed) => {
                self.action.right();
            }
            Event::Key(Key::Up, ButtonState::Pressed) => {
                self.action.jump();
            }
            Event::Key(Key::Z, ButtonState::Pressed) => {
                self.action.take();
            }
            Event::Key(Key::X, ButtonState::Pressed) => {
                self.action.drop();
            }
            Event::Key(Key::Escape, ButtonState::Pressed) => {
                window.close();
            }
            _ => (),
        }

        Ok(())
    }

    fn draw(&mut self, window: &mut Window) -> Result<()> {
        let render = Render::new(window, self.assets.clone());
        self.sys.run(render);
        Ok(())
    }
}

pub fn run_client() {
    lifecycle::run::<Screen>("Chintama", Vector::new(800, 600), Settings::default());
}

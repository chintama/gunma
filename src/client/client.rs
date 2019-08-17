use super::screen::Screen;
use log::*;
use quicksilver::{geom::Vector, lifecycle, lifecycle::Settings};

pub fn run_client() {
    info!("Running client");
    lifecycle::run::<Screen>("Chintama", Vector::new(800, 600), Settings::default());
}

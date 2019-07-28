#[macro_use]
pub mod vector;

mod client;
pub mod components;
mod config;
mod entities;
pub mod error;
mod gui;
pub mod protocol;
pub mod resources;
mod systems;

pub use crate::config::Config;
pub use crate::gui::run_gui;
pub use crate::systems::run_world;

#[macro_use]
mod vector;

pub mod components;
pub mod entities;
pub mod protocol;
pub mod resources;

mod client;
mod config;
mod error;
mod io;
mod systems;

pub use crate::config::{Config, ConfigBuilder};
pub use crate::error::Result;
pub use crate::systems::Systems;
pub use crate::vector::Vector;

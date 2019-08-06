#[macro_use]
mod vector;

pub mod components;
pub mod entities;
pub mod events;
pub mod protocol;
pub mod resources;

mod client;
mod collide;
mod config;
mod error;
mod io;
mod systems;

pub mod prelude {
    pub use crate::entities::CreateEntity;
}

pub use crate::config::{Config, ConfigBuilder};
pub use crate::error::Result;
pub use crate::io::Io;
pub use crate::systems::Systems;
pub use crate::vector::Vector;

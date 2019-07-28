use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Config {
    pub game_server: String,
    pub terrain_server: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            game_server: "ws://127.0.0.1:8080/ws/".into(),
            terrain_server: "ws://127.0.0.1:8080/ws/".into(),
        }
    }
}

#[derive(Default, Clone, Debug)]
pub struct ConfigBuilder {
    cfg: Config,
}

impl Config {
    pub fn build() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}

impl ConfigBuilder {
    pub fn game_server(mut self, path: &str) -> Self {
        self.cfg.game_server = path.into();
        self
    }

    pub fn terrain_server(mut self, path: &str) -> Self {
        self.cfg.terrain_server = path.into();
        self
    }

    pub fn build(self) -> Config {
        self.cfg
    }
}

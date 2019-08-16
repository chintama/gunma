use econf::LoadEnv;

#[derive(LoadEnv, Debug, Default)]
pub struct Config {
    pub terrain_server: Option<String>,
    pub game_server: Option<String>,
    pub class: u64,
}

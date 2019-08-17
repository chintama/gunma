use crate::systems::Systems;
use log::*;

pub fn run_server() {
    info!("Running server");

    let mut sys = Systems::new().unwrap();
    sys.server_start();

    loop {
        std::thread::sleep(std::time::Duration::from_millis(16));
        sys.server_update();
    }
}

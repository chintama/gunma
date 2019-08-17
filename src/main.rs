use gunma::{run_client, run_server};
use structopt::StructOpt;

#[derive(StructOpt)]
struct ServerConfig {
    #[structopt(name = "host", default_value = "ws://127.0.0.1:8980/ws/")]
    host: String,
}

#[derive(StructOpt)]
enum Opt {
    #[structopt(name = "client")]
    Client,
    #[structopt(name = "server")]
    Server(ServerConfig),
}

fn main() {
    env_logger::init();

    let opt = Opt::from_args();

    match opt {
        Opt::Client => {
            run_client();
        }
        Opt::Server(cfg) => {
            run_server();
        }
    }
}

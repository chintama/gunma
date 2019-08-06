use crate::{
    client::Client,
    components::*,
    config::Config,
    error::{Error, Result},
    events::*,
    protocol::*,
};
use log::*;

pub struct Io {
    game_client: Option<Client>,
    terrain_client: Client,
}

impl Io {
    pub fn new(cfg: Config) -> Result<Self> {
        let game_client = match &cfg.game_server {
            Some(addr) => Some(Client::new(addr)?),
            None => None,
        };
        let terrain_client = Client::new(&cfg.terrain_server)?;

        Ok(Self {
            game_client,
            terrain_client,
        })
    }

    pub fn is_client(&self) -> bool {
        self.game_client.is_some()
    }

    pub fn login(&mut self, cls: Class) -> Result<LoginAck> {
        self.game_client
            .as_mut()
            .expect("Server tries to login")
            .send(Message::Login(Login { cls }))?;

        match self.game_client.as_mut().unwrap().recv()? {
            Message::LoginAck(ack) => Ok(ack),
            msg => {
                error!("Invalid response to login: {:?}", msg);
                Err(Error::LoginError.into())
            }
        }
    }

    pub fn send_event(&mut self, ev: Event) -> Result<()> {
        self.game_client
            .as_mut()
            .expect("Server tries to send action")
            .send(Message::Event(ev))
    }

    pub fn recv_events(&mut self, max: usize) -> Result<Vec<Event>> {
        let mut items = Vec::new();

        for _ in 0..max {
            match self.game_client.as_mut().unwrap().try_recv()? {
                Some(Message::Event(item)) => {
                    items.push(item);
                }
                None => {
                    break;
                }
                e => {
                    warn!("Received invalid message: {:?}", e);
                    break;
                }
            }
        }

        Ok(items)
    }

    pub fn get_all_terrain(&mut self) -> Result<Vec<Terrain>> {
        self.terrain_client.send(Message::GetAllTerrain)?;

        let mut items = Vec::new();

        loop {
            match self.terrain_client.recv()? {
                Message::Terrain(t) => {
                    info!("Received terrain from server: {:?}", t);

                    items.push(t);
                }
                Message::EndTerrain => break,
                msg => warn!("Invalid message: {:?}", msg),
            }
        }

        Ok(items)
    }
}

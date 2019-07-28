use crate::{error::Result, protocol::Message};
use log::*;
use std::{
    sync::mpsc::{channel, Receiver, Sender, TryRecvError},
    thread::{spawn, JoinHandle},
};
use websocket::client::ClientBuilder;
use websocket::{receiver::Reader, sender::Writer, stream::sync::TcpStream, OwnedMessage};

pub struct Client {
    wr_tx: Sender<OwnedMessage>,
    rd_rx: Receiver<OwnedMessage>,
    _wr_thread: JoinHandle<()>,
    _rd_thread: JoinHandle<()>,
}

impl Client {
    pub fn new(s: &str) -> Result<Self> {
        let client = ClientBuilder::new(s)?
            .add_protocol("rust-websocket")
            .connect_insecure()?;

        let (rd_tx, rd_rx) = channel();
        let (wr_tx, wr_rx) = channel();
        let (ws_rx, ws_tx) = client.split()?;
        let wr_tx2 = wr_tx.clone();
        let _wr_thread = spawn(move || {
            wr_loop(ws_tx, wr_rx).unwrap();
        });
        let _rd_thread = spawn(move || {
            rd_loop(ws_rx, wr_tx2, rd_tx).unwrap();
        });

        Ok(Self {
            wr_tx,
            rd_rx,
            _wr_thread,
            _rd_thread,
        })
    }

    pub fn send(&mut self, msg: Message) -> Result<()> {
        let msg = OwnedMessage::Binary(serde_json::to_vec(&msg)?);
        Ok(self.wr_tx.send(msg)?)
    }

    pub fn try_recv(&mut self) -> Result<Option<Message>> {
        match self.rd_rx.try_recv() {
            Ok(OwnedMessage::Binary(msg)) => Ok(Some(serde_json::from_slice(&msg)?)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(e) => Err(e.into()),
            _ => unreachable!(),
        }
    }

    pub fn recv(&mut self) -> Result<Message> {
        match self.rd_rx.recv() {
            Ok(OwnedMessage::Binary(msg)) => Ok(serde_json::from_slice(&msg)?),
            Err(e) => Err(e.into()),
            _ => unreachable!(),
        }
    }
}

fn wr_loop(mut ws_tx: Writer<TcpStream>, rx: Receiver<OwnedMessage>) -> Result<()> {
    loop {
        let msg = match rx.recv() {
            Ok(msg) => msg,
            Err(_) => break,
        };

        match msg {
            OwnedMessage::Close(_) => {
                ws_tx.send_message(&msg)?;
                break;
            }
            msg => ws_tx.send_message(&msg)?,
        }
    }

    Ok(())
}

fn rd_loop(
    mut ws_rx: Reader<TcpStream>,
    ws_tx: Sender<OwnedMessage>,
    tx: Sender<OwnedMessage>,
) -> Result<()> {
    for msg in ws_rx.incoming_messages() {
        match msg {
            Ok(OwnedMessage::Close(_)) => {
                ws_tx.send(OwnedMessage::Close(None))?;
            }
            Ok(OwnedMessage::Ping(data)) => {
                ws_tx.send(OwnedMessage::Pong(data))?;
            }
            Ok(OwnedMessage::Binary(data)) => {
                tx.send(OwnedMessage::Binary(data))?;
            }
            Ok(msg) => warn!("Received unsupported message: {:?}", msg),
            Err(e) => {
                warn!("Receive error: {}", e);
                ws_tx.send(OwnedMessage::Close(None))?;
                break;
            }
        }
    }

    Ok(())
}

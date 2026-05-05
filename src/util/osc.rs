use std::net::{SocketAddr, UdpSocket};

use rosc::{OscMessage, OscPacket, OscType, encoder};

use crate::{constants, util::error::AppError};

pub struct OscClient {
    socket: UdpSocket,
    address: String,
}

impl OscClient {
    pub fn new (bind: SocketAddr, target: SocketAddr) -> std::io::Result<Self> {
        Ok(Self {
            socket: UdpSocket::bind(bind)?,
            address: target.to_string(),
        })
    }

    // Send a VRChat ChatBox OSC Request
    pub fn send_osc(&self, text: &str, send_immediately: bool, play_sfx: bool) -> Result<(), AppError> {
        let message = OscMessage {
            addr: constants::ENDPOINT.to_string(),
            args: vec![
                OscType::String(text.to_string()),  // Message
                OscType::Bool(send_immediately),                // Send Immediate
                OscType::Bool(play_sfx),               // Notify SFX
            ],
        };

        let packet = OscPacket::Message(message);
        let buffer = encoder::encode(&packet)?;

        self.socket.send_to(&buffer, &self.address)?;
        Ok(())
    }
}
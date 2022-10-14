use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BasePacket {
    pub packet_type: String,
    pub packet_addr: String,
    pub packet_data: String,
}

impl BasePacket {
    pub fn new(packet_type: String, packet_addr: String, packet_data: String) -> BasePacket {
        BasePacket {
            packet_type,
            packet_addr,
            packet_data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LoginPacket {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogoutPacket {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DirectionalPacket {
    pub x: f32,
    pub y: f32,
}

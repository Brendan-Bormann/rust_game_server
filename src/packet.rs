use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BasePacket {
    pub packet_type: String,
    pub packet_data: String,
}

impl BasePacket {
    pub fn new(packet_type: String, packet_data: String) -> BasePacket {
        BasePacket {
            packet_type,
            packet_data,
        }
    }
}

pub struct GamePacket {
    pub packet_type: String,
    pub packet_data: String,
    pub player_id: String,
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

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

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_string(packet_string: &str) -> BasePacket {
        serde_json::from_str(packet_string).unwrap()
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerCommandPacket {
    pub command_type: String,
    pub command_data: String,
}

impl PlayerCommandPacket {
    pub fn new(command_type: String, command_data: String) -> PlayerCommandPacket {
        PlayerCommandPacket {
            command_type,
            command_data,
        }
    }

    pub fn to_string(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }

    pub fn from_string(packet_string: &str) -> PlayerCommandPacket {
        serde_json::from_str(packet_string).unwrap()
    }
}

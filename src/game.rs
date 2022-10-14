use crate::packets;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Game {
    pub id_counter: u32,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id_counter: 0,
            players: vec![],
        }
    }

    pub fn game_tick(&mut self) {
        self.move_players();
    }

    pub fn move_players(&mut self) {
        for i in 0..self.players.len() {
            if self.players[i].direction.x == 0.0 && self.players[i].direction.y == 0.0 {
                return;
            }
            self.players[i].position.x += self.players[i].direction.x * self.players[i].stats.speed;
            self.players[i].position.y += self.players[i].direction.y * self.players[i].stats.speed;
        }
    }

    pub fn handle_packet(&mut self, packet: packets::BasePacket) {
        match packet.packet_type.as_str() {
            "login" => self.login_player(&packet.packet_data, &packet.packet_addr),
            "logout" => self.logout_player(&packet.packet_data, &packet.packet_addr),
            "directional" => self.set_player_directional(&packet.packet_data, &packet.packet_addr),
            _ => {
                println!("Received unknown packet!")
            }
        }
    }

    pub fn get_player_index(&mut self, player_addr: &str) -> Option<usize> {
        return self
            .players
            .iter()
            .position(|player| player.addr == player_addr);
    }

    pub fn get_players_online(&mut self) -> i32 {
        let mut count = 0;
        for player in self.players.iter() {
            if player.online {
                count += 1;
            }
        }
        return count;
    }

    pub fn login_player(&mut self, packet_data: &str, player_addr: &str) {
        let packet: packets::LoginPacket = serde_json::from_str(&packet_data).unwrap();
        let index = self.get_player_index(player_addr);

        match index {
            Some(index) => {
                self.players[index].addr = player_addr.to_owned();
                self.players[index].online = true;
            }
            None => {
                self.id_counter += 1;

                let new_player = Player::new(
                    self.id_counter,
                    player_addr.to_string(),
                    String::from(&packet.username),
                    packet.password,
                );

                self.players.push(new_player);
            }
        }

        println!(
            "[{}] has logged in - {} players online",
            &packet.username,
            self.get_players_online()
        );
    }

    pub fn logout_player(&mut self, packet_data: &str, player_addr: &str) {
        let packet: packets::LoginPacket = serde_json::from_str(&packet_data).unwrap();
        let index = self
            .get_player_index(player_addr)
            .expect("Failed to find player to logout");

        self.players[index].online = false;

        println!(
            "[{}] has logged out - {} players online",
            &packet.username,
            self.get_players_online()
        );
    }

    pub fn set_player_directional(&mut self, packet_data: &str, player_addr: &str) {
        let packet: packets::DirectionalPacket = serde_json::from_str(&packet_data).unwrap();
        let player_index = self
            .get_player_index(&player_addr)
            .expect("Player not found for directional update");

        self.players[player_index].direction = Vector2 {
            x: packet.x,
            y: packet.y,
        };
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Vector2 {
    x: f32,
    y: f32,
}

impl Vector2 {
    pub fn new() -> Vector2 {
        Vector2 { x: 0.0, y: 0.0 }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Player {
    pub id: u32,
    pub online: bool,
    pub addr: String,
    pub timeout: u32,
    pub username: String,
    pub password: String,
    pub position: Vector2,
    pub direction: Vector2,
    pub stats: PlayerStats,
}

impl Player {
    pub fn new(id: u32, addr: String, username: String, password: String) -> Player {
        Player {
            id,
            online: true,
            addr,
            timeout: 0,
            username,
            password,
            direction: Vector2::new(),
            position: Vector2::new(),
            stats: PlayerStats {
                health: 0,
                speed: 1.0,
            },
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerStats {
    pub health: i32,
    pub speed: f32,
}

use crate::{network, packet};
use serde::{Deserialize, Serialize};
use std::{
    alloc::System,
    time::{Duration, SystemTime},
};

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

            println!(
                "Player {} moved to x: {} y: {}",
                self.players[i].username, self.players[i].position.x, self.players[i].position.y
            )
        }
    }

    pub fn handle_command(&mut self, player_command: PlayerCommand) {
        match player_command.command_type.as_str() {
            "login" => println!("Received login command"),
            "logout" => println!("Received logout command"),
            "directional" => println!("Received directional command"),
            _ => println!("Received unknown command type"),
        }
    }

    pub fn get_player_index(&mut self, player_id: &str) -> Option<usize> {
        return self
            .players
            .iter()
            .position(|player| player.id == player_id);
    }

    pub fn get_players_online(&mut self) -> i32 {
        return self.players.len() as i32;
    }

    pub fn login_player(&mut self, player_id: &str, packet_data: &str) {
        let packet: packet::LoginPacket = serde_json::from_str(&packet_data).unwrap();
        let index = self.get_player_index(player_id);

        match index {
            Some(index) => {
                println!("Uh oh")
            }
            None => {
                let new_player = Player::new(
                    player_id.to_string(),
                    String::from(&packet.username),
                    String::from(&packet.password),
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

    pub fn logout_player(&mut self, player_id: &str, packet_data: &str) {
        let packet: packet::LoginPacket = serde_json::from_str(&packet_data).unwrap();
        let index = self
            .get_player_index(player_id)
            .expect("Failed to find player to logout");

        self.players.remove(index);

        println!(
            "[{}] has logged out - {} players online",
            &packet.username,
            self.get_players_online()
        );
    }

    pub fn set_player_directional(&mut self, player_id: &str, packet_data: &str) {
        let packet: packet::DirectionalPacket = serde_json::from_str(&packet_data).unwrap();
        let player_index = self
            .get_player_index(&player_id)
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
    pub id: String,
    pub username: String,
    pub password: String,
    pub position: Vector2,
    pub direction: Vector2,
    pub stats: PlayerStats,
}

impl Player {
    pub fn new(id: String, username: String, password: String) -> Player {
        Player {
            id,
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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PlayerCommand {
    pub client_id: String,
    pub command_type: String,
    pub command_data: String,
}

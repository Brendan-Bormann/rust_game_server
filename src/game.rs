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
        println!("Received command: {:?}", player_command);
        match player_command.command_type.as_str() {
            "login" => {
                self.login_player(player_command.client_id, &player_command.command_data);
            }
            "logout" => {
                println!("Received logout command");
            }
            "directional" => {
                println!("Received directional command");
            }
            _ => {
                println!("Received unknown command type");
            }
        }
    }

    pub fn generate_player_id(&mut self) -> String {
        self.id_counter += 1;
        return self.id_counter.to_string();
    }

    pub fn create_player(
        &mut self,
        client_id: String,
        username: String,
        password: String,
    ) -> Player {
        let new_player = Player::new(self.generate_player_id(), username, password);
        self.players.push(new_player.clone());
        return new_player;
    }

    pub fn attach_client_to_player(&mut self, player_index: usize, client_id: String) {
        self.players[player_index].client_id = Some(client_id.clone());
    }

    pub fn get_player(&mut self, player_id: String) -> Option<(Player, usize)> {
        let client_index = self
            .players
            .iter()
            .position(|player| player.id == player_id.clone());

        return match client_index {
            Some(index) => Some((self.players[index].clone(), index)),
            None => None,
        };
    }

    pub fn get_player_by_client(&mut self, client_id: String) -> Option<(Player, usize)> {
        let client_index = self
            .players
            .iter()
            .position(|player| player.client_id == Some(client_id.clone()));

        return match client_index {
            Some(index) => Some((self.players[index].clone(), index)),
            None => None,
        };
    }

    pub fn get_player_by_username(&mut self, username: &str) -> Option<(Player, usize)> {
        let client_index = self
            .players
            .iter()
            .position(|player| player.username == username.clone());

        return match client_index {
            Some(index) => Some((self.players[index].clone(), index)),
            None => None,
        };
    }

    pub fn get_players_online(&mut self) -> i32 {
        return self.players.len() as i32;
    }

    pub fn already_logged_in(&mut self, client_id: String) -> bool {
        for player in &self.players {
            if player.client_id == Some(client_id.clone()) {
                return true;
            }
        }

        return false;
    }

    pub fn login_player(&mut self, client_id: String, command_data: &str) -> Option<Player> {
        let login_command: LoginCommand = serde_json::from_str(&command_data).unwrap();

        if self.already_logged_in(client_id.clone()) {
            println!("Client already logged in");
            return None;
        };

        match self.get_player_by_username(&login_command.username) {
            Some((player, index)) => {
                if player.password != login_command.password {
                    println!("Incorrect password for player");
                    return None;
                }

                self.attach_client_to_player(index, client_id.clone());

                println!("[{}] has logged in, welcome back", &player.username);

                return Some(player);
            }
            None => {
                let new_player = self.create_player(
                    client_id.clone(),
                    login_command.username,
                    login_command.password,
                );
                let (_, index) = self.get_player(new_player.id.clone()).unwrap();
                self.attach_client_to_player(index, client_id.clone());

                println!(
                    "[{}] has logged in for the first time!",
                    &new_player.username
                );

                return Some(new_player);
            }
        }
    }

    pub fn logout_player(&mut self, player_id: &str, packet_data: &str) {
        let packet: packet::LoginPacket = serde_json::from_str(&packet_data).unwrap();
        let (_, index) = self
            .get_player(player_id.to_string())
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
        let (_, index) = self
            .get_player(player_id.to_string())
            .expect("Player not found for directional update");

        self.players[index].direction = Vector2 {
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
    pub client_id: Option<String>,
    pub username: String,
    pub password: String,
    pub position: Vector2,
    pub direction: Vector2,
    pub stats: PlayerStats,
    pub active: bool,
}

impl Player {
    pub fn new(id: String, username: String, password: String) -> Player {
        Player {
            id,
            client_id: None,
            username,
            password,
            direction: Vector2::new(),
            position: Vector2::new(),
            stats: PlayerStats {
                health: 10,
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

impl PlayerCommand {
    pub fn new(client_id: String, command_type: String, command_data: String) -> PlayerCommand {
        PlayerCommand {
            client_id,
            command_type: String::from("unknown"),
            command_data,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct LoginCommand {
    pub username: String,
    pub password: String,
}

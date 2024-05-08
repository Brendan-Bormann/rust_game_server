use std::{
    net::UdpSocket,
    time::{Duration, SystemTime},
};

use crate::packet;

#[derive(Clone, Debug)]
pub struct Client {
    pub id: String,
    pub addr: String,
    pub last_packet_time: SystemTime,
    pub player_id: Option<String>,
}

impl Client {
    pub fn new(id: String, addr: String) -> Client {
        Client {
            id,
            addr,
            last_packet_time: SystemTime::now(),
            player_id: None,
        }
    }
}

#[derive(Debug)]
pub struct Server {
    pub clients: Vec<Client>,
    pub socket: UdpSocket,
    pub server_addr: String,
    pub client_id_counter: u32,
    pub timeout_duration_ms: u32,
}

impl Server {
    pub fn new(server_addr: &str) -> Server {
        let new_socket = UdpSocket::bind(server_addr).unwrap();
        new_socket.set_nonblocking(true).unwrap();
        new_socket
            .set_write_timeout(Some(Duration::from_millis(10)))
            .unwrap();
        println!("- Network started @ {}", server_addr);

        Server {
            clients: Vec::new(),
            socket: new_socket,
            server_addr: server_addr.to_string(),
            client_id_counter: 0,
            timeout_duration_ms: 5000,
        }
    }

    pub fn check_socket_error(&self) {
        match self.socket.take_error() {
            Ok(Some(error)) => println!("[SOCKET] error: {:?}", error),
            Ok(_) => {}
            Err(error) => println!("[SOCKET] take_error failed: {:?}", error),
        }
    }

    pub fn generate_client_id(&mut self) -> String {
        self.client_id_counter += 1;
        return self.client_id_counter.to_string();
    }

    pub fn get_client_count(&self) -> usize {
        return self.clients.len();
    }

    pub fn client_logged_in(&self, client_id: String) -> bool {
        return self.get_client(client_id).unwrap().0.player_id.is_some();
    }

    pub fn create_client(&mut self, addr: String) -> Client {
        let client_id = self.generate_client_id();
        let new_client = Client::new(client_id, addr);
        self.clients.push(new_client.clone());

        println!("Client connected [total: {}]", self.get_client_count());

        return new_client;
    }

    pub fn remove_client(&mut self, client_id: String) {
        let index = self
            .clients
            .iter()
            .position(|client| client.id == client_id)
            .expect("Failed to find connection to remove");

        self.clients.remove(index);

        println!("Client disconnected [total: {}]", self.get_client_count());
    }

    /**
     * Returns a tuple containing a client clone and the index of the client in the clients vector
     */
    pub fn get_client(&self, client_id: String) -> Option<(Client, usize)> {
        let client_index = self
            .clients
            .iter()
            .position(|client| client.id == client_id);

        return match client_index {
            Some(index) => Some((self.clients[index].clone(), index)),
            None => None,
        };
    }

    pub fn check_for_addr(&self, addr: String) -> Option<Client> {
        let client_index = self.clients.iter().position(|client| client.addr == addr);

        return match client_index {
            Some(index) => Some(self.clients[index].clone()),
            None => None,
        };
    }

    pub fn send_packet_to_client(
        &self,
        client_id: String,
        packet_string: String,
    ) -> Result<usize, usize> {
        let (client, _) = self.get_client(client_id).unwrap();
        let packet_bytes = packet_string.as_bytes();

        match self.socket.send_to(&packet_bytes, client.addr) {
            Ok(_) => return Ok(packet_bytes.len()),
            Err(e) => {
                println!("Failed to send packet to client: {}", e.to_string());
                return Err(0);
            }
        }
    }

    pub fn send_packet_to_all_clients(&self, packet_string: String) -> Result<usize, usize> {
        let mut total_bytes_sent = 0;

        for client in self.clients.iter() {
            match self.send_packet_to_client(client.id.clone(), packet_string.clone()) {
                Ok(bytes_sent) => total_bytes_sent += bytes_sent,
                Err(_) => continue,
            }
        }

        return Ok(total_bytes_sent);
    }

    pub fn check_clients_for_timeout(&mut self) {
        let current_time = SystemTime::now();

        let mut clients_to_remove = Vec::new();

        for client in self.clients.iter() {
            let time_diff = current_time
                .duration_since(client.last_packet_time)
                .unwrap()
                .as_millis();

            if time_diff > self.timeout_duration_ms as u128 {
                clients_to_remove.push(client.clone());
            }
        }

        for client in clients_to_remove.iter() {
            self.remove_client(client.id.clone());
        }
    }

    pub fn reset_client_timeout(&mut self, client_id: String) {
        let (_, index) = self.get_client(client_id).unwrap();
        self.clients[index].last_packet_time = SystemTime::now();
    }

    pub fn check_for_packets(&self) -> bool {
        let mut buffer = vec![0u8; 4096];

        return match self.socket.peek(&mut buffer) {
            Ok(bytes) => bytes > 0,
            Err(_) => false,
        };
    }

    pub fn receive_packet_from(&self) -> Option<(packet::BasePacket, String)> {
        let mut buffer = vec![0u8; 4096];

        match self.socket.recv_from(&mut buffer) {
            Ok((data_len, addr)) => {
                let (trimmed_buffer, _) = &buffer[..].split_at(data_len);
                let buffer_string = String::from_utf8_lossy(&trimmed_buffer).to_string();

                match serde_json::from_str(&buffer_string) {
                    Ok(result) => return Some((result, addr.to_string())),
                    Err(_) => return None,
                };
            }
            Err(_) => {
                return None;
            }
        }
    }
}

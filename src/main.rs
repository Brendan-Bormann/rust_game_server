use std::net::UdpSocket;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::{thread, thread::sleep};

mod game;
mod packets;

const SERV_ADDR: &str = "127.0.0.1:8080";

fn main() {
    println!("\nServer started...");

    let socket = UdpSocket::bind(SERV_ADDR).unwrap();
    println!("Server bound to {}", SERV_ADDR);
    let socket_clone = socket.try_clone().unwrap();

    let (gs_sender, gs_receiver): (Sender<game::Game>, Receiver<game::Game>) = channel();
    let (pi_sender, pi_receiver): (Sender<packets::BasePacket>, Receiver<packets::BasePacket>) =
        channel();

    thread::spawn(move || {
        // game state and loop
        println!("Game created");
        let mut game_state = game::Game::new();

        loop {
            sleep(Duration::from_millis(100));
            game_state.game_tick();

            match pi_receiver.recv_timeout(Duration::from_millis(0)) {
                Ok(packet) => game_state.handle_packet(packet),
                Err(_) => {}
            }

            gs_sender.send(game_state.clone()).unwrap();
        }
    });

    thread::spawn(move || {
        // incoming packets
        println!("Listening for packets...");
        packet_listener(socket, pi_sender);
    });

    thread::spawn(move || {
        // out going packets
        println!("Ready to send packets!");
        loop {
            let game_state = gs_receiver.recv().unwrap();
            let game_state_string = serde_json::to_string(&game_state).unwrap();
            let game_state_bytes = game_state_string.as_bytes();

            game_state.players.iter().for_each(|player| {
                socket_clone
                    .send_to(&game_state_bytes, &player.addr)
                    .unwrap();
            });
        }
    });

    loop {}
}

fn packet_listener(socket: UdpSocket, pi_sender: Sender<packets::BasePacket>) {
    loop {
        let mut buffer = vec![0u8; 4096];
        let (data_len, addr) = socket
            .recv_from(&mut buffer)
            .expect("Failed to receive message on UDP socket.");
        let (trimmed_buffer, _) = &buffer[..].split_at(data_len);
        let buffer_string = String::from_utf8_lossy(&trimmed_buffer).to_string();

        println!("Received data!");

        let mut packet: packets::BasePacket = match serde_json::from_str(&buffer_string) {
            Ok(result) => result,
            Err(e) => {
                println!("{}", e);
                continue;
            }
        };

        packet.packet_addr = addr.to_string();
        pi_sender.send(packet).unwrap();

        match socket.send_to("awk".as_bytes(), addr) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}

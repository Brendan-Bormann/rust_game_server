use std::net::UdpSocket;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{thread, thread::sleep};

use packet::BasePacket;

mod game;
mod network;
mod packet;

const SERV_ADDR: &str = "127.0.0.1:5000";

fn main() {
    println!("\n--- Server started ---");
    let mut server = network::Server::new(SERV_ADDR);

    let (state_sender, state_receiver): (Sender<game::Game>, Receiver<game::Game>) = channel();
    let (player_command_sender, player_command_receiver): (
        Sender<game::PlayerCommand>,
        Receiver<game::PlayerCommand>,
    ) = channel();

    thread::spawn(move || {
        println!("- Game world started");

        let mut game = game::Game::new();

        loop {
            sleep(Duration::from_millis(100));
            game.game_tick();

            match player_command_receiver.recv_timeout(Duration::from_millis(0)) {
                Ok(player_command) => game.handle_command(player_command),
                Err(_) => {}
            }

            state_sender.send(game.clone()).unwrap();
        }
    });

    loop {
        server.check_socket_error();

        let current_state = state_receiver.recv().unwrap();
        let current_state_string = serde_json::to_string(&current_state).unwrap();

        server
            .send_packet_to_all_clients(current_state_string)
            .expect("Failed to send state to clients");

        if server.check_for_packets() {
            println!("Packet received!");

            let (packet, addr) = server.receive_packet_from().unwrap();

            let client = match server.check_for_addr(addr.clone()) {
                Some(client) => client,
                None => server.create_client(addr.clone()),
            };

            server.reset_client_timeout(client.id.clone());

            handle_packet(&player_command_sender, client, packet);
        }

        server.check_clients_for_timeout();
    }
}

fn handle_packet(
    player_command_sender: &Sender<game::PlayerCommand>,
    client: network::Client,
    packet: packet::BasePacket,
) {
    match packet.packet_type.as_str() {
        "login" => println!("Login packet received from Client [{}]", client.id),
        "logout" => println!("Logout packet received from Client [{}]", client.id),
        "directional" => println!("Directional packet received from Client [{}]", client.id),
        _ => {
            println!(
                "Received unknown packet! Type: [{}] from Client [{}]",
                packet.packet_type, client.id
            );
        }
    }
}

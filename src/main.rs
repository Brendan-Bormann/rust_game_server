use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;
use std::{thread, thread::sleep};

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

        // println!("server {:?}", server);

        let current_state = state_receiver.recv().unwrap();
        let current_state_string = serde_json::to_string(&current_state).unwrap();

        if server.check_for_packets() {
            let (packet, addr) = server.receive_packet_from().unwrap();

            let client = match server.check_for_addr(addr.clone()) {
                Some(client) => client,
                None => server.create_client(addr.clone()),
            };

            server.reset_client_timeout(client.id.clone());

            println!("Received packet from: {} {:?}", client.id.clone(), packet);

            match packet.packet_type.as_str() {
                "state" => {
                    let current_state_string_clone = current_state_string.clone();
                    server
                        .send_packet_to_client(client.id.clone(), current_state_string_clone)
                        .unwrap();
                    continue;
                }
                "command" => {
                    let packet_data: packet::PlayerCommandPacket =
                        match serde_json::from_str(&packet.packet_data) {
                            Ok(result) => result,
                            Err(_) => continue,
                        };

                    let player_command = game::PlayerCommand {
                        client_id: client.id.clone(),
                        command_type: packet_data.command_type,
                        command_data: packet_data.command_data,
                    };

                    player_command_sender.send(player_command).unwrap();
                    continue;
                }
                _ => println!("Received unknown packet!"),
            }
        }

        server.check_clients_for_timeout();
    }
}

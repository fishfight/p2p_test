use std::io::{self, BufRead};

use nanoserde::{DeBin, SerBin};

const RELAY_ADDR: &str = "173.0.157.169:35000";

#[derive(Debug, DeBin, SerBin)]
pub enum Message {
    /// Empty message, used for connection test
    Idle,
    RelayRequestId,
    RelayIdAssigned(u64),
    RelayConnectTo(u64),
    RelayConnected,
    Payload(u32),
}

fn main() {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();

    println!("Select connection type");
    println!("  - 1: LAN");
    println!("  - 2: STUN");
    println!("  - 3: Relay");
    println!("For types description visit https://github.com/fishfight/FishFight/blob/main/docs/multiplayer.md");

    let connection_kind = io::stdin().lock().lines().next().unwrap().unwrap();

    let self_addr;
    match connection_kind.as_str() {
        "1" => {
            self_addr = format!("{}", socket.local_addr().unwrap());
            socket.set_nonblocking(true).unwrap();
        }
        "2" => {
            let sc = stunclient::StunClient::with_google_stun_server();
            self_addr = format!("{}", sc.query_external_address(&socket).unwrap());
            socket.set_nonblocking(true).unwrap();
        }
        "3" => {
            socket.connect(RELAY_ADDR).unwrap();

            socket.set_nonblocking(true).unwrap();

            loop {
                let _ = socket.send(&nanoserde::SerBin::serialize_bin(&Message::RelayRequestId));

                let mut buf = [0; 100];
                if socket.recv(&mut buf).is_ok() {
                    let message: Message =
                        nanoserde::DeBin::deserialize_bin(&buf[..]).ok().unwrap();
                    if let Message::RelayIdAssigned(id) = message {
                        self_addr = format!("{}", id);
                        break;
                    }
                }
            }
        }
        _ => {
            panic!("Unknown connection type, please type 1, 2 or 3!");
        }
    }

    println!("Socket created");
    println!("Your address, share this with other player: {}", self_addr);

    println!("Input other player IP and press Enter:");

    let other_addr = io::stdin().lock().lines().next().unwrap().unwrap();

    println!("IP entered: {}", other_addr);

    match connection_kind.as_str() {
        "1" | "2" => socket.connect(&other_addr).unwrap(),
        "3" => {
            let other_id = other_addr.parse::<u64>().unwrap();
            loop {
                let _ = socket.send(&nanoserde::SerBin::serialize_bin(&Message::RelayConnectTo(
                    other_id,
                )));

                let mut buf = [0; 100];
                if socket.recv(&mut buf).is_ok() {
                    let message: Message =
                        nanoserde::DeBin::deserialize_bin(&buf[..]).ok().unwrap();
                    if let Message::RelayConnected = message {
                        break;
                    }
                }
            }
        }
        _ => {
            unreachable!()
        }
    }

    println!("connected");

    socket
        .send(&nanoserde::SerBin::serialize_bin(&Message::Payload(66)))
        .unwrap();

    loop {
        socket
            .send(&nanoserde::SerBin::serialize_bin(&Message::Payload(66)))
            .unwrap();

        let mut buf = [0; 100];
        match socket.recv(&mut buf) {
            Ok(count) => {
                let message: Message = nanoserde::DeBin::deserialize_bin(&buf[..]).unwrap();
                println!("Success, recvd! {}, {:?}", count, message)
            }
            Err(..) => {}
        }
    }
}

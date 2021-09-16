use std::io::{self, BufRead};

fn main() {
    let socket = std::net::UdpSocket::bind("0.0.0.0:0").unwrap();

    //let self_addr = format!("{}", socket.local_addr().unwrap());

    let sc = stunclient::StunClient::with_google_stun_server();
    let self_addr = format!("{}", sc.query_external_address(&socket).unwrap());

    println!("Your IP, share this with other player: {}", self_addr);

    println!("Input other player IP and press Enter:");

    let other_addr = io::stdin().lock().lines().next().unwrap().unwrap();

    println!("IP entered: {}", other_addr);
    socket.connect(&other_addr).unwrap();

    loop {
        println!("connected");
        socket.send(&[66]).unwrap();
        println!("sended, waiting for other player's test data");

        let mut buf = [0; 2];
        socket.recv(&mut buf).unwrap();

        println!("Success, recvd! {:?}", buf);
    }
}

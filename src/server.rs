use std::net::{Ipv4Addr, SocketAddrV4, UdpSocket};

fn get_local_ipaddress() -> Option<String> {
    let socket = match UdpSocket::bind("0.0.0.0:0") {
        Ok(s) => s,
        Err(_) => return None,
    };

    match socket.connect("8.8.8.8:80") {
        Ok(()) => (),
        Err(_) => return None,
    };

    match socket.local_addr() {
        Ok(addr) => return Some(addr.ip().to_string()),
        Err(_) => return None,
    };
}

fn main() {
    let local_addr = get_local_ipaddress().unwrap_or("127.0.0.1".to_owned());
    let port = 42069;

    match igd::search_gateway(Default::default()) {
        Err(ref err) => println!("Error: {}", err),
        Ok(gateway) => {
            let local_addr = local_addr.parse::<Ipv4Addr>().unwrap();
            let local_addr = SocketAddrV4::new(local_addr, port);

            match gateway.add_port(
                igd::PortMappingProtocol::UDP,
                port,
                local_addr,
                60,
                "add_port example",
            ) {
                Err(ref err) => {
                    println!("There was an error! {}", err);
                }
                Ok(()) => {
                    println!("It worked");
                }
            }
        }
    }

    println!("binding to {:?}", local_addr);
    let socket = UdpSocket::bind((local_addr, port)).unwrap();
    socket
        .set_nonblocking(true)
        .expect("Can't set non-blocking mode");
    let mut read_buffer = [0u8; 4];
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        loop {
            if let Ok((n, addr)) = socket.recv_from(&mut read_buffer) {
                println!("recv {} bytes from {:?}", n, addr);
                println!("read_buffer: {:?}", read_buffer);
            } else {
                break;
            }
        }

        println!("rustlin");
    }
}

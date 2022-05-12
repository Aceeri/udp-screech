use link_conditioner::*;

use std::time::Instant;

fn main() {
    let server_ip = my_internet_ip::get().unwrap();
    let port = 42069;
    let server = format!("{}:{}", server_ip, port);
    println!("connecting to {:?}", server);
    let socket = UdpConditioner::bind(("0.0.0.0", 0)).unwrap();
    //socket.connect(&server).expect("Can't connect to server");
    let mut increment = 0;
    loop {
        std::thread::sleep(std::time::Duration::from_millis(5000));
        socket.send_to(&[increment], &server).unwrap();
        println!("{:?}: sent: {:?}", Instant::now(), increment);
        increment += 1;
    }
}

/*
Instant { t: 2117691.5324617s }: recv: [12]
Instant { t: 2117690.5305415s }: sent: 12
 */

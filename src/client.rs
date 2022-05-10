use std::net::UdpSocket;

fn main() {
    let server_ip = my_internet_ip::get().unwrap();
    let port = 42069;
    let server = format!("{}:{}", server_ip, port);
    println!("connecting to {:?}", server);
    let socket = UdpSocket::bind(("0.0.0.0", 0)).unwrap();
    socket.connect(&server).expect("Can't connect to server");
    loop {
        std::thread::sleep(std::time::Duration::from_millis(1000));
        socket.send_to(&[0, 1, 2, 3], &server).unwrap();
        println!("rustlin");
    }
}

use std::{
    io::{Read, Write},
    net::TcpStream,
};

use http::IP_ADDRESS;

fn main() {
    let mut stream = TcpStream::connect(IP_ADDRESS).unwrap();
    stream
        .write_all("Hello, server! I am Khang Tran".as_bytes())
        .unwrap();
    let mut buffer = [0; 1024];
    let _read = stream.read(&mut buffer).unwrap();

    println!("Received from server: {}", String::from_utf8_lossy(&buffer));
}

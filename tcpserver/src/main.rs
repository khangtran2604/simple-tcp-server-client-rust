use http::IP_ADDRESS;
use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() {
    match TcpListener::bind(IP_ADDRESS) {
        Ok(listener) => {
            println!("Server is running on {}", listener.local_addr().unwrap());

            for stream in listener.incoming() {
                let mut stream = stream.unwrap();
                println!("Has new connection");

                let mut buffer = [0; 1024];
                let _read_bytes = stream.read(&mut buffer).unwrap();
                println!("Received: {}", String::from_utf8_lossy(&buffer));

                stream
                    .write_all("Hello, client! This is response from me(server)!".as_bytes())
                    .unwrap();
            }
        }
        Err(e) => {
            println!("Failed to bind to {}: {}", IP_ADDRESS, e);
            panic!("Failed to bind to address");
        }
    }
}

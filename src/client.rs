use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str;

fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read from stdin");

        stream.write_all(input.as_bytes()).expect("Failed to write to stream");

        let mut buffer = [0; 1024];
        let mut bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        while bytes_read ==0 {
            bytes_read = stream.read(&mut buffer).expect("Failed to read from stream")
        }
        println!("Received: {}", str::from_utf8(&buffer[..bytes_read]).expect("Failed to read buffer"));
    }
}

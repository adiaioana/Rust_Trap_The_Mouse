mod board;
mod prereq;

use std::net::TcpStream;
use std::io::{self, Read, Write};
use std::str;
use board::{Board,gameboard_state};
fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:7878").expect("Failed to connect to server");
    let mut nothing_to_read_next_time=false;
    loop {
        if nothing_to_read_next_time==false {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read from stdin");
    
            stream.write_all(input.as_bytes()).expect("Failed to write to stream");

        }   
        nothing_to_read_next_time=false;
        let mut buffer = [0; 1024];
        let mut bytes_read = stream.read(&mut buffer).expect("Failed to read from stream");
        while bytes_read ==0 {
            bytes_read = stream.read(&mut buffer).expect("Failed to read from stream")
        }
        
        let input = match str::from_utf8(&buffer[..bytes_read]) {
            Ok(v) => v.trim(),
            Err(_) => {
                continue;
            },
        };
        if input.contains('{') {
            nothing_to_read_next_time=true;
            let mut GB:Board=Board::new();
            GB.translate_to_board(String::from(input));
            GB.print_for_debug();
        }
        else if input.contains("Game") {
            nothing_to_read_next_time=true;
        }
        println!("Received: {}", str::from_utf8(&buffer[..bytes_read]).expect("Failed to read buffer"));
    }
}

use std::io::prelude::*;
use std::net::TcpStream;
use std::env;

fn main() {

    let args: Vec<_> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: cargo run ipaddr:portnr username");
        std::process::exit(1);
    }

    let ip: &str = args[1].as_ref();
    println!("Connecting to {:?}",ip);
    let mut stream = TcpStream::connect(ip).unwrap();

    let username: &[u8] = args[2].as_bytes();
    match stream.write(&username) {
        Ok(size) => println!("Sent {} bytes.",size),
        Err(e) => println!("Failed sending message: {}!", e),
    }
    
    let mut reply = [0; 128];

    match stream.read(&mut reply) {
        Ok(_) => {
            println!("Server says: {}", std::str::from_utf8(&reply).unwrap());
        },
        Err(e) => println!("Failed reading response: {}!", e),
    }
} 


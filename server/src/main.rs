use std::net;
use std::thread;
use std::io::Write;
use std::io::Read;

fn main()
{
    println!("Welcome to Tokyo Rust Chat Server\n");
        
    let ip = net::Ipv4Addr::new(0, 0, 0, 0);
    let port = 55555;
    let addr = net::SocketAddrV4::new(ip, port);

    let listener = net::TcpListener::bind(addr).unwrap();

    println!("[Start listening for connections]");
    for stream in listener.incoming() {
        println!("[Got incoming connection, spawn thread]");
        match stream {
            Ok(stream) => {
                thread::spawn(move|| {
                    handle_client(stream)
                });
            }
            Err(e) => println!("[Connection failed: {}!]", e),
        }
    }    
}

fn handle_client(mut stream: net::TcpStream)
{
    let response = b"Welcome";
    let mut message = [0; 128];
 
    match stream.read(&mut message) {
        Ok(size_read) => {
            println!(">{} logs in.", std::str::from_utf8(&message[0..size_read]).unwrap());
            match stream.write(response) {
                Ok(size_sent) => println!("[Sent {} bytes reply to {}]", size_sent, std::str::from_utf8(&message[0..size_read]).unwrap()),
                Err(e) => println!("[Failed sending message: {}!]", e),
            }
        },
        Err(e) => println!("[Failed sending response: {}!]", e),
    }
}

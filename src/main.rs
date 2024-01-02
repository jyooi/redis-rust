use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};


fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    
    
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                
                    handle_connection(&mut stream);
                }
                Err(e) => {
                    println!("error: {}", e);
                    break;
                }
            }
        }
    
}

fn handle_connection(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(_size) => {
                let response = "+PONG\r\n";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            },
            Err(_e) => {
                // Handle any error that might occur during read
                break;
            },
        }
    }
}
use std::io::Write;
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind to port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("error: {}", { e });
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let pong = b"+PONG\r\n";
    stream.write_all(pong).expect("Failed to write to stream");
}

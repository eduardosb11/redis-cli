use std::io::{Read, Write};
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
    let mut buffer = vec![0; 512];
    loop {
        let bytes_read = stream
            .read(&mut buffer)
            .expect("Failed to read from stream");
        if bytes_read == 0 {
            break;
        }
        let pong = b"+PONG\r\n";
        stream.write_all(pong).expect("Failed to write to stream");
    }
}

use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").expect("Could not bind to port");
    listener
        .set_nonblocking(true)
        .expect("Cannot set non-blocking");

    let mut clients: Vec<TcpStream> = Vec::new();

    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                stream
                    .set_nonblocking(true)
                    .expect("set_nonblocking call failed");
                clients.push(stream);
            }
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {}
            Err(_e) => {}
        }

        for mut client in &clients {
            handle_client(&mut client);
        }
    }
}

fn handle_client(mut stream: &TcpStream) {
    let mut buffer = vec![0; 512];
    let pong = b"+PONG\r\n";
    match stream.read(&mut buffer) {
        Ok(0) => {
            return;
        }
        Ok(_) => {
            stream.write_all(pong).expect("Failed to write");
        }
        Err(e) if e.kind() == io::ErrorKind::WouldBlock => {}
        Err(_) => {}
    }
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

/// Handles a single client connection. This is the same as in step-01.
fn handle_client(mut stream: TcpStream) {
    println!("Client connected on thread: {:?}", thread::current().id());
    let mut buffer = [0; 1024];
    loop {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Client disconnected.");
                    break;
                }
                if let Err(e) = stream.write_all(&buffer[0..bytes_read]) {
                    eprintln!("Failed to write to stream: {}", e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}

/// The main function now spawns a new thread for each incoming connection.
fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    println!("Server listening on {}", address);

    // `listener.incoming()` is an iterator that blocks on `accept()`.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // A new client has connected. Instead of handling it in the
                // main thread, we spawn a new thread to handle it.
                // The `move` keyword transfers ownership of the `stream` variable
                // from the main thread to the new thread.
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

/// Handles a single client connection.
///
/// It reads data from the stream and writes it back to the client,
/// effectively "echoing" the message. The loop continues until the
/// client closes the connection, at which point `read` will return 0 bytes.
///
/// # Arguments
///
/// * `stream` - A mutable reference to the TcpStream of the client.
fn handle_client(mut stream: TcpStream) {
    println!("Client connected: {}", stream.peer_addr().unwrap());
    let mut buffer = [0; 1024]; // A 1KB buffer
    loop {
        // The read call is blocking. The thread will wait here until data is
        // received from the client.
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                // If read returns 0, it means the client has closed the connection.
                if bytes_read == 0 {
                    println!("Client disconnected.");
                    break;
                }

                // Echo the data back to the client.
                // The write_all call is also blocking.
                if let Err(e) = stream.write_all(&buffer[0..bytes_read]) {
                    eprintln!("Failed to write to stream: {}", e);
                    break;
                }
                println!("Echoed {} bytes.", bytes_read);
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                break;
            }
        }
    }
}

/// The main function binds a TcpListener to an address and enters a loop
/// to accept incoming connections.
fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;

    println!("Server listening on {}", address);

    // The `incoming()` method returns an iterator over the connections being
    // received on this listener.
    // The `accept()` call is blocking. The main thread will wait here until
    // a client tries to connect.
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // For each valid connection, we call handle_client.
                // **CRITICAL LIMITATION**: The server can only handle one client
                // at a time. `handle_client` must complete before the next
                // client connection can be accepted.
                handle_client(stream);
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}

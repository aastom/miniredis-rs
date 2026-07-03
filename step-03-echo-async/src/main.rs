use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Handles a single client connection asynchronously.
///
/// It reads data from the stream and writes it back to the client. The `.await`
/// keyword is used to pause the task execution until an I/O operation is
/// ready, allowing other tasks to run on the same thread.
async fn handle_client(mut stream: TcpStream) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Client connected: {}", peer_addr);

    let mut buffer = [0; 1024];

    loop {
        // The `read` method on a `tokio::net::TcpStream` is async.
        // We must `.await` it. This will yield control back to the Tokio
        // runtime if there is no data to be read.
        match stream.read(&mut buffer).await {
            // `Ok(0)` means the client has closed the connection.
            Ok(0) => {
                println!("Client {} disconnected.", peer_addr);
                return;
            }
            Ok(n) => {
                // Echo the data back to the client.
                if let Err(e) = stream.write_all(&buffer[0..n]).await {
                    eprintln!("Failed to write to stream: {}", e);
                    return;
                }
            }
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return;
            }
        }
    }
}

/// The `#[tokio::main]` attribute is a macro that transforms the `async fn main`
/// into a synchronous `fn main` that initializes a Tokio runtime and runs the
/// async main function.
#[tokio::main]
async fn main() -> io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).await?;

    println!("Server listening on {}", address);

    loop {
        // The `accept` method on a `tokio::net::TcpListener` is also async.
        // It yields control until a new connection is available.
        let (stream, _) = listener.accept().await?;

        // A new client has connected. Instead of spawning a thread, we spawn
        // a Tokio task. Tokio tasks are much more lightweight than OS threads.
        // The `handle_client` function is now an `async fn`.
        tokio::spawn(handle_client(stream));
    }
}

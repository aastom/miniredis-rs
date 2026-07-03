use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};
use step_04_kv_logic::{Command, SharedState, new_shared_state};

/// Handles a client connection, now with application logic.
async fn handle_client(mut stream: TcpStream, db: SharedState) {
    let peer_addr = stream.peer_addr().unwrap();
    println!("Client connected: {}", peer_addr);

    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                println!("Client {} disconnected.", peer_addr);
                return;
            }
            Ok(n) => {
                let command = Command::from_buffer(&buffer[0..n]);
                let response = match command {
                    Command::Set { key, value } => {
                        // To modify the database, we must first acquire a lock.
                        let mut db_lock = db.lock().await;
                        db_lock.insert(key, value);
                        "OK\n".to_string()
                    }
                    Command::Get { key } => {
                        // To read from the database, we also need a lock.
                        let db_lock = db.lock().await;
                        match db_lock.get(&key) {
                            Some(value) => format!("{}\n", value),
                            None => "NULL\n".to_string(),
                        }
                    }
                    Command::Invalid => "INVALID COMMAND\n".to_string(),
                };

                // Send the response back to the client.
                if let Err(e) = stream.write_all(response.as_bytes()).await {
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

#[tokio::main]
async fn main() -> io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address).await?;
    println!("KV Store server listening on {}", address);

    // Create the shared state (our in-memory database).
    let db = new_shared_state();

    loop {
        let (stream, _) = listener.accept().await?;

        // For each connection, clone the Arc to the shared state.
        // This increments the reference count, allowing the new task to
        // share ownership of the state.
        let db_clone = db.clone();
        
        tokio::spawn(async move {
            handle_client(stream, db_clone).await;
        });
    }
}

use std::env;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("Usage: client <COMMAND> [ARGS...]");
        eprintln!("Commands:");
        eprintln!("  GET <key>");
        eprintln!("  SET <key> <value>");
        return Ok(());
    }

    // Construct the command string to send to the server.
    // We add a newline at the end as our protocol is newline-delimited.
    let command = args[1..].join(" ");
    let command_with_newline = format!("{}\n", command);

    // Connect to the server.
    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;

    // Write the command to the server.
    stream.write_all(command_with_newline.as_bytes()).await?;

    // Read the server's response.
    let mut buffer = vec![0; 1024];
    let n = stream.read(&mut buffer).await?;
    let response = String::from_utf8_lossy(&buffer[..n]);

    // Print the response to standard output.
    print!("{}", response);

    Ok(())
}

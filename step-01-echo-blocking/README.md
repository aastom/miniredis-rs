# Step 1: Blocking Echo Server

## Goal

The goal of this step is to build the simplest possible TCP server using only the Rust standard library. This server will accept a single client connection, echo back any data it receives, and wait for the next client after the first one disconnects.

This provides a "quick win" and establishes a foundational mental model of what a TCP socket is and how the basic `bind`/`accept`/`read`/`write` lifecycle works.

## Key Concepts Introduced

*   **`std::net::TcpListener`**: A socket server. We use `::bind()` to attach it to an address and port, and then `.incoming()` to listen for connections.
*   **`std::net::TcpStream`**: Represents a TCP connection between a local socket and a remote socket. It implements the `Read` and `Write` traits.
*   **`std::io::Read` and `std::io::Write` traits**: These traits provide the core methods for reading from and writing to I/O streams.
*   **Blocking I/O**: Notice that calls like `listener.incoming().next()` and `stream.read()` will pause the execution of the thread until a connection is available or data is received. This is the simplest form of I/O, but has significant limitations.

## How to Run This Code

1.  **Start the server:**
    ```bash
    cargo run
    ```
    The server will start and print `Server listening on 127.0.0.1:8080`.

2.  **Connect with a client:** You can use a simple tool like `netcat` (`nc`) to act as the client. Open a new terminal and run:
    ```bash
    nc 127.0.0.1 8080
    ```
    Now, type any message and press Enter. The server will receive the message and send it right back to your `netcat` client.

3.  **To disconnect:** Press `Ctrl+C` in the `netcat` terminal.

## Limitations & Next Steps

This server has a major flaw: **it can only handle one client at a time.**

When `handle_client()` is running, the main loop with `listener.incoming()` is blocked. A second client attempting to connect will be forced to wait until the first client disconnects and `handle_client()` returns.

This makes the server unusable for any real-world application. How do we solve this? We need to handle each client concurrently.

**Next -> [Step 2: Thread-per-Client Server](../step-02-echo-threaded/)**: We will address this limitation by spawning a new thread for every client that connects.

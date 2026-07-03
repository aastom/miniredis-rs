# Step 2: Thread-per-Client Echo Server

## Goal

The goal of this step is to solve the "one client at a time" problem from Step 1. We will evolve the server to handle multiple clients concurrently by spawning a dedicated operating system thread for each incoming connection.

This demonstrates a classic, albeit dated, model for network concurrency and highlights the trade-offs involved.

## Key Concepts Introduced

*   **`std::thread::spawn`**: This function creates a new OS thread and runs the provided closure within it. This allows the main `accept` loop to immediately go back to waiting for new connections while the new thread handles the existing client.
*   **`move` closures**: The `move` keyword before the closure `|| { ... }` is critical. It forces the closure to take ownership of the variables it uses from the environment, such as the `stream`. Without `move`, the `stream` would remain owned by the main loop, but the new thread needs its own ownership to work with it.

## How to Run This Code

1.  **Start the server:**
    ```bash
    cargo run
    ```
2.  **Connect multiple clients:** Open several new terminal windows and run `nc 127.0.0.1:8080` in each.
    ```bash
    # Terminal 1
    nc 127.0.0.1:8080
    # Terminal 2
    nc 127.0.0.1:8080
    # Terminal 3
    nc 127.0.0.1:8080
    ```
    You can now type messages in any of the client terminals, and the server will echo them back independently. The server logs will show that different threads are handling each connection.

## Limitations & Next Steps

This model works, but it's inefficient and doesn't scale well. Here's why:

*   **Resource Intensive**: OS threads are heavyweight. They have significant memory overhead and require a "context switch" by the OS kernel to switch between them, which is slow. A server with 10,000 clients would require 10,000 threads, which would overwhelm most systems.
*   **Wasted Time**: Most of the time, our `handle_client` function is just sitting idle, blocked on `stream.read()`, waiting for the client to send data. While it's waiting, the entire thread is asleep and its resources are tied up, even though it's doing no work.

This is a classic "C10k problem". How can we handle thousands of connections without thousands of threads? The answer is **asynchronous I/O**.

**Next -> [Step 3: Async Echo Server with Tokio](../step-03-echo-async/)**: We will replace our threads with lightweight async "tasks" and introduce the `tokio` runtime to manage them efficiently.

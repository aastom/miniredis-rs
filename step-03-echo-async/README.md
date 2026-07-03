# Step 3: Asynchronous Echo Server with Tokio

## Goal

The goal is to solve the scalability problem of the thread-per-client model. We will rebuild the echo server using the modern, industry-standard `tokio` runtime. This introduces the concept of `async`/`await` and non-blocking I/O.

## Key Concepts Introduced

*   **`async` and `await`**: These are new keywords in Rust that allow you to write asynchronous code that looks and feels like synchronous code.
    *   An `async fn` is a function that returns a `Future`. A `Future` is a value that might not be computed yet.
    *   The `.await` keyword is used to pause the execution of an `async fn` until its `Future` is resolved, without blocking the entire thread.
*   **The Tokio Runtime (`#[tokio::main]`)**: `async` functions can't run themselves. They need an executor, or "runtime", to poll their `Future`s and drive them to completion. `tokio` is the most popular runtime for Rust. The `#[tokio::main]` macro conveniently sets up a runtime for us.
*   **`tokio::net`**: Tokio provides its own asynchronous versions of `std::net` components like `TcpListener` and `TcpStream`. Their methods (`accept`, `read`, `write`) are `async` and must be `.await`ed.
*   **`tokio::spawn`**: This is Tokio's equivalent of `thread::spawn`. It spawns a lightweight, asynchronous "task" (sometimes called a green thread) that is managed by the Tokio runtime. Millions of these can exist at once, as they don't map 1:1 to OS threads.

## How to Run This Code

1.  **Add the dependency**: This crate is the first to have an external dependency. `tokio` is listed in `Cargo.toml`.
2.  **Start the server:**
    ```bash
    cargo run
    ```
3.  **Connect multiple clients:** Just like in Step 2, open several terminals and connect with `nc`. The server will handle all of them concurrently, but far more efficiently than the threaded model.

## Limitations & Next Steps

We now have a highly scalable echo server. It can handle thousands of concurrent connections efficiently. However, it doesn't *do* anything interesting yet. It just shuttles bytes back and forth.

The stated goal is to build a key-value store. This involves:
1.  **Parsing a protocol**: We need to interpret the bytes we receive as commands (e.g., `SET key value` or `GET key`).
2.  **Managing state**: We need a place to store the data (like a `HashMap`) and a way to share it safely between all the concurrent tasks.

**Next -> [Step 4: Application Logic](./step-04-kv-logic/)**: We will step away from the networking code to focus on building the core application logic in a separate library crate.

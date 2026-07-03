# Step 4: Application Logic - Protocol and State

## Goal

In this step, we move away from the networking layer to define and build the core logic of our application. This involves two key pieces:
1.  **A Protocol Parser**: A function to translate the raw bytes from the network into structured commands that our application can understand.
2.  **A State Manager**: A data structure to hold our key-value data that can be safely accessed from multiple concurrent tasks.

This work is done in a dedicated library crate (`step-04-kv-logic`) to enforce a clean separation of concerns. This logic doesn't know anything about networking, making it easy to test and reason about in isolation.

## Key Concepts Introduced

*   **Separation of Concerns**: By putting our application logic in a library, we separate it from the I/O handling in our server binary. The server's job is to move bytes; the library's job is to understand what those bytes mean.
*   **Protocol Definition**: We define a simple text-based protocol: `SET key value` and `GET key`. Our parser is a simple function that operates on byte slices (`&[u8]`).
*   **`Arc<T>` (Atomically Referenced Counter)**: This is a smart pointer that allows us to have multiple "owners" of a piece of data. We need this because every spawned task for a client connection will need a reference to our central database. `Arc` safely handles the reference counting across threads/tasks.
*   **`tokio::sync::Mutex<T>`**: A standard `HashMap` is not safe to be modified by multiple tasks at once. A `Mutex` (Mutual Exclusion) provides a locking mechanism to ensure that only one task has access to the data at any given time.
    *   **Why `tokio::sync::Mutex`?** Unlike `std::sync::Mutex`, if a Tokio task tries to acquire a lock that's already held, it will `await` the lock, yielding control back to the scheduler. This allows other tasks to run. A `std::sync::Mutex` would block the entire OS thread, which is disastrous for an async runtime.
*   **`Arc<Mutex<T>>` Pattern**: This combination is the cornerstone of sharing mutable state in asynchronous (and multithreaded) Rust. `Arc` allows shared ownership, and `Mutex` allows safe mutation.

## How to Use This Code

This is a library crate, not a runnable server. You can't `cargo run` it. However, you can run its tests to verify the logic:
```bash
cargo test
```
The tests confirm that our command parser works correctly and that the shared state can be safely manipulated.

## Limitations & Next Steps

We now have the "brains" of our application. We have a way to parse commands and a thread-safe data structure to store our information.

What's missing is the "body" - the server that will accept connections and use this logic.

**Next -> [Step 5: The Async Key-Value Store](../step-05-kv-server/)**: We will integrate this logic library into our async echo server from Step 3 to create the final, fully-functional application.

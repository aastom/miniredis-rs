# Step 5: The Complete Async Key-Value Store

## Goal

This is the capstone step. We will combine the scalable async server from Step 3 with the application logic from Step 4 to create the final product: a complete, concurrent, asynchronous key-value store.

## Key Concepts Introduced

*   **Integration**: This step is all about integrating the two major components (`tokio` networking and our logic crate).
*   **`Arc::clone()`**: Before spawning a task for a new client, we call `.clone()` on our `Arc<Mutex<...>>`. This is cheap—it doesn't clone the `HashMap`, it only increments a reference counter. This gives the new task shared ownership of the database handle, allowing it to access the state alongside all other tasks.
*   **`.lock().await`**: Inside `handle_client`, before we can use the `HashMap`, we must call `.lock().await`. This asynchronously waits for the mutex lock to be available. Once the `MutexGuard` returned by `.lock()` goes out of scope at the end of the `match` arm, the lock is automatically released, allowing other waiting tasks to proceed.

## How to Run This Code

1.  **Start the server:**
    ```bash
    cargo run
    ```
    The server will start on `127.0.0.1:8080`.

2.  **Interact with the client:** We've also built a simple client application. Navigate to the `client` directory in a new terminal and use it to send commands:
    ```bash
    # From the ../client directory
    
    # Set a key
    cargo run -- SET mykey "hello world"
    # Server should respond with OK

    # Get the key
    cargo run -- GET mykey
    # Client should print "hello world"
    
    # Get a non-existent key
    cargo run -- GET anotherkey
    # Client should print NULL
    ```
You can run multiple client commands from different terminals to see the server handle them concurrently.

## The Journey is Complete!

Congratulations! You have progressed from a simple, single-client blocking server to a sophisticated, scalable, and safe asynchronous application.

You have learned:
*   The basics of TCP sockets.
*   The limitations of blocking I/O and the thread-per-client model.
*   The power and syntax of `async`/`await` with Tokio.
*   The critical importance of separating network I/O from application logic.
*   The standard pattern for managing shared, mutable state in concurrent Rust: `Arc<Mutex<T>>`.

From here, you have a solid foundation to explore more advanced topics like persistence, more complex async patterns, and other network protocols.

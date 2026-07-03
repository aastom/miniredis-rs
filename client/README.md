# Client Application

## Goal

This crate provides a simple command-line interface (CLI) client to interact with our key-value servers.

## How to Use

The client is run from your terminal using `cargo run`. The arguments you provide after `--` are sent to the server as a command.

The client is located in the `client/` directory.

**Prerequisites:** One of the servers (`step-03`, `step-05`, etc.) must be running.

### Examples

**SET a key-value pair:**
```bash
# The value is quoted to be treated as a single argument
cargo run -- SET mykey "hello from the client"
```
The server should respond with `OK`.

**GET a key:**
```bash
cargo run -- GET mykey
```
The client will print the value returned by the server: `hello from the client`.

**GET a non-existent key:**
```bash
cargo run -- GET non-existent-key
```
The client will print the `NULL` value returned by the server.

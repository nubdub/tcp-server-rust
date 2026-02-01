# Rust Networking

A simple TCP server implementation in Rust demonstrating basic socket programming and multi-threaded connection handling.

## Overview

This project implements a basic TCP server that listens for incoming client connections and responds with a simple greeting message. Each client connection is handled in a separate thread to allow for concurrent connections.

## Features

- **TCP Server**: Listens on `127.0.0.1:8080` for incoming connections
- **Multi-threaded**: Handles each client connection in a separate thread for concurrent request processing
- **Simple Protocol**: Receives client requests and responds with a greeting message

## Project Structure

```
rust-networking/
├── Cargo.toml           # Project metadata and dependencies
├── README.md            # This file
└── src/
    ├── main.rs          # Entry point - initializes and starts the server
    └── tcp.rs           # TCP server implementation
```

## Building

Make sure you have Rust installed. Then, build the project using Cargo:

```bash
cargo build
```

For an optimized release build:

```bash
cargo build --release
```

## Running

Start the server with:

```bash
cargo run
```

The server will start listening on `127.0.0.1:8080`:

```
Server listening on 127.0.0.1:8080
```

## Testing

To test the server, you can use `telnet` or `nc` (netcat) in another terminal:

### Using telnet:
```bash
telnet 127.0.0.1 8080
```

Then type any message and press Enter. You should receive:
```
Hello, Client!
```

### Using netcat:
```bash
echo "Hello Server" | nc 127.0.0.1 8080
```

### Using curl:
```bash
curl telnet://127.0.0.1:8080
```

## How It Works

1. **Server Initialization** (`main.rs`): Sets up the server on `127.0.0.1:8080`
2. **Connection Listening** (`tcp.rs`): The server continuously listens for incoming connections
3. **Client Handling**: Each new connection is spawned in a new thread to handle multiple clients concurrently
4. **Request Processing**: The server reads up to 1024 bytes from each client, prints the request, and sends back "Hello, Client!"

## Dependencies

This project uses only Rust's standard library with no external dependencies.

## Requirements

- Rust 1.70+ (or compatible version)
- Cargo package manager

## License

This is a sample project for learning purposes.

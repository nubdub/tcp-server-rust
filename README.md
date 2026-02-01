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
├── Dockerfile           # Docker configuration for containerization
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

### Locally

Start the server with:

```bash
cargo run
```

The server will start listening on `127.0.0.1:8080`:

```
Server listening on 127.0.0.1:8080
```

### Using Docker

#### Building the Docker Image

Build the Docker image:

```bash
docker build -t rust-networking .
```

You can also specify a version tag:

```bash
docker build -t rust-networking:1.0 .
```

#### Running the Container

Run the container with port mapping:

```bash
docker run -p 8080:8080 rust-networking
```

To run in detached mode (background):

```bash
docker run -d -p 8080:8080 --name rust-server rust-networking
```

To view logs from a running container:

```bash
docker logs rust-server
```

To stop the container:

```bash
docker stop rust-server
```

To remove the container:

```bash
docker rm rust-server
```

The server will be accessible on `localhost:8080` from your host machine.

#### Dockerfile Overview

The project uses a multi-stage Docker build:

1. **Builder Stage** (`rust:latest`): Compiles the Rust project in release mode
2. **Runtime Stage** (`debian:bookworm-slim`): Runs the compiled binary with minimal dependencies

This approach significantly reduces the final image size by excluding the Rust compiler and build artifacts from the production image.

## Docker Details

### Image Specifications

- **Base Image (Builder)**: `rust:latest` - Used to compile the project
- **Base Image (Runtime)**: `debian:bookworm-slim` - Lightweight Debian image for production
- **Exposed Port**: 8080 (TCP)
- **Working Directory**: `/app`

### Building Options

For faster builds during development, you can use a custom Dockerfile. However, the default setup is optimized for production deployment.

### Testing with Docker

To test the containerized server:

```bash
# Build and run in one command
docker run -p 8080:8080 $(docker build -q .)
```

Or use the published image:

```bash
docker run -p 8080:8080 rust-networking
```

Then test with telnet, netcat, or curl as described in the Testing section.

### Network Considerations

- The server inside the Docker container listens on `127.0.0.1:8080`
- When accessed from outside the container, use `localhost:8080` or your machine's IP address
- The `-p 8080:8080` flag maps port 8080 from the container to your host machine

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

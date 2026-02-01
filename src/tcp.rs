// Importing necessary modules from the Rust library
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    // buffer to read data from client, 1024 bytes
    let mut buffer = [0; 1024];
    // read data from the stream and stores in buffer
    stream.read(&mut buffer).expect("Failed to read from client!");

    // convert data in buffer to UTF-8 encoded string
    // from_utf8_lossy handles invalid UTF-8 sequences gracefully
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);
    // prepare a response to send back to the client
    let response = "Hello, Client!".as_bytes();

    // write the response back to the client
    stream.write(response).expect("Failed to write response to client!");
}

pub(crate) fn start_server(address: &str) {
    // bind the TCP listener to the specified address
    let listener = TcpListener::bind(address).expect("Failed to bind to address!");
    println!("Server listening on {}", address);

    // accept incoming connections in a loop
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // handle the client connection in a separate thread
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                // eprintln prints to stderr
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
}
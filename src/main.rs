mod tcp;
use crate::tcp::start_server;

fn main() {
    let address = "127.0.0.1:8080";
    start_server(address);
}

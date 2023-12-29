pub mod server;
mod private_server;

pub fn connect() {
    println!("network::connect");
    private_server::connect();
}
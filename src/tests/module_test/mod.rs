pub fn test() {
    println!("[module test]");
    client::connect();
    network::connect();
    network::server::connect();
    // network::private_server::connect();  // pub 이 안붙은 모듈이라 여기서 접근 불가
}

pub mod client;
pub mod network;
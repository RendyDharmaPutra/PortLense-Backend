mod core;
mod services;

use services::get_ports_service;

#[tokio::main]
async fn main() {
    let ports = get_ports_service().await;
    for port in ports {
        println!("{}://{}:{}", port.protocol, port.address, port.port);
    }
}
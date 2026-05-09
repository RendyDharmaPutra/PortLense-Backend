mod core;

use core::get_used_ports;

#[tokio::main]
async fn main() {
    let ports = get_used_ports().await;
    for port in ports {
        println!("{:?}://{}:{}", port.protocol, port.ip, port.port);
    }
}
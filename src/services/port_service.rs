use serde::Serialize;
use crate::core::network::port_scanner::get_used_ports;

#[derive(Serialize)]
pub struct PortResponse {
    pub protocol: String,
    pub address: String,
    pub port: u16,
}

pub async fn get_ports_service() -> Vec<PortResponse> {
    let ports = get_used_ports().await;
    
    ports
        .into_iter()
        .map(|p| PortResponse {
            protocol: format!("{:?}", p.protocol),
            address: p.ip,
            port: p.port,
        })
        .collect()
}
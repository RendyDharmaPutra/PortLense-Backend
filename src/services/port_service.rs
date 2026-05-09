use crate::core::network::port_scanner::{PortInfo, get_used_ports};

pub type PortResponse = PortInfo;

pub async fn get_ports_service() -> Vec<PortResponse> {
    get_used_ports().await
}
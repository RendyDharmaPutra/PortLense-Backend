use netstat2::error::Error;
use crate::core::network::port_scanner::{PortInfo, get_used_ports};

pub async fn get_ports_service() -> Result<Vec<PortInfo>, Error> {
    get_used_ports().await
}
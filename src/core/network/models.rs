use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum NetworkProtocol {
    TCP,
    UDP,
}

#[derive(Serialize)]
pub struct PortInfo {
    pub protocol: NetworkProtocol,
    pub ip: String,
    pub port: u16,
}

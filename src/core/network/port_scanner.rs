
use serde::Serialize;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};


#[derive(Debug, Serialize)]
pub enum NetworkProtocol {
    TCP,
    UDP,
}

#[derive(Debug, Serialize)]
pub struct PortInfo {
    pub protocol: NetworkProtocol,
    pub ip: String,
    pub port: u16,
}


pub async fn get_used_ports() -> Vec<PortInfo> {
    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    let mut result = Vec::new();

    if let Ok(sockets) = get_sockets_info(af_flags, proto_flags) {
        for socket in sockets {
            // ? Debug
            // println!("{:?}", socket.protocol_socket_info);

            match socket.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp) => {
                    result.push(PortInfo {
                        protocol: NetworkProtocol::TCP,
                        ip: tcp.local_addr.to_string(),
                        port: tcp.local_port,
                    });
                }
                ProtocolSocketInfo::Udp(udp) => {
                    result.push(PortInfo {
                        protocol: NetworkProtocol::UDP,
                        ip: udp.local_addr.to_string(),
                        port: udp.local_port,
                    });
                }
            }
        }
    }

    result
}
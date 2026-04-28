use serde::Serialize;
use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo};


#[derive(Serialize)]
struct PortInfo {
    protocol: String,
    ip: String,
    port: u16,
}

async fn get_ports() -> Vec<PortInfo> {
    let mut result = vec![];

    let af_flags = AddressFamilyFlags::IPV4 | AddressFamilyFlags::IPV6;
    let proto_flags = ProtocolFlags::TCP | ProtocolFlags::UDP;

    if let Ok(sockets) = get_sockets_info(af_flags, proto_flags) {
        for s in sockets {
            // Debug
            // println!("{:?}", s.protocol_socket_info);

            match s.protocol_socket_info {
                ProtocolSocketInfo::Tcp(tcp) => {
                    result.push(PortInfo {
                        protocol: "TCP".into(),
                        ip: tcp.local_addr.to_string(),
                        port: tcp.local_port,
                    });
                }
                ProtocolSocketInfo::Udp(udp) => {
                    result.push(PortInfo {
                        protocol: "UDP".into(),
                        ip: udp.local_addr.to_string(),
                        port: udp.local_port,
                    });
                }
            }
        }
    }

    result
}

#[tokio::main]
async fn main() {
    let ports = get_ports().await;
    for port in ports {
        println!("{}://{}:{}", port.protocol, port.ip, port.port);
    }
}
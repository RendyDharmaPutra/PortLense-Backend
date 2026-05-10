use axum::Json;
use serde::Serialize;
use crate::services::port_service::{get_ports_service};
use crate::core::network::port_scanner::PortInfo;

#[derive(Debug, Serialize)]
pub struct PortResponse {
    pub success: bool,
    pub message: String,
    pub data: Option<Vec<PortInfo>>,
}

pub async fn ports_handler() -> Json<PortResponse> {
    let result = get_ports_service().await;

    match result {
        Ok(ports) => Json(PortResponse {
            success: true,
            message: "Ports fetched successfully".to_string(),
            data: Some(ports),
        }),
        Err(e) => Json(PortResponse {
            success: false,
            message: e.to_string(),
            data: None,
        }),
    }
}

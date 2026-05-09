use axum::Json;
use crate::services::port_service::{get_ports_service, PortResponse};

pub async fn ports_handler() -> Json<Vec<PortResponse>> {
    Json(get_ports_service().await)
}

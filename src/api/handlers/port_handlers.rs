use crate::services::{get_ports_service, PortResponse};
use axum::Json;

pub async fn ports_handler() -> Json<Vec<PortResponse>> {
    Json(get_ports_service().await)
}

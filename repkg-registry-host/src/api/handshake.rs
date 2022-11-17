use axum::{http::StatusCode, response::IntoResponse, Json};
use repkg_core::protocols::server as protocols;
use tracing::info;

pub async fn handshake() -> impl IntoResponse {
    let response = protocols::handshake::response();
    info!("/api/handshake recieved, response: {:?}", response);

    (StatusCode::ACCEPTED, Json(response))
}

use axum::{http::StatusCode, response::IntoResponse, Json};
use repkg_core::protocols::server as protocols;

pub async fn handshake() -> impl IntoResponse {
    let response = protocols::handshake::response();

    (StatusCode::ACCEPTED, Json(response))
}

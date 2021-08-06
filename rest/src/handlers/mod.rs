use axum::http::StatusCode;

pub mod beans;

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}

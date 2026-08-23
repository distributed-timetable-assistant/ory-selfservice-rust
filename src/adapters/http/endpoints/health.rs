use http::StatusCode;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}
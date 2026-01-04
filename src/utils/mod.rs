pub mod jwt;
pub mod middleware;
pub mod keycloak;

pub fn handle_error<'a, T: std::fmt::Debug>(err: T) -> (axum::http::StatusCode, &'a str) {
    tracing::error!("{:?}", err);
    (axum::http::StatusCode::INTERNAL_SERVER_ERROR, "internal server error")
}

pub mod jwt;
pub mod middleware;
pub mod keycloak;
pub mod extractor;

pub fn handle_error<'a, T: std::fmt::Debug>(err: T) -> (axum::http::StatusCode, &'a str) {
    tracing::error!("{:?}", err);
    (
        axum::http::StatusCode::INTERNAL_SERVER_ERROR,
        "internal server error"  // &'static str
    )
}

async fn extract_state<T: axum::extract::FromRef<crate::app::App>>(
    parts: &mut axum::http::request::Parts,
    state: &crate::app::App,
) -> Result<T, axum::response::Response> {
    <axum::extract::State<T> as axum::extract::FromRequestParts<crate::app::App>>::from_request_parts(parts, state)
        .await
        .map(|v| v.0)
        .map_err(handle_error)
        .map_err(|err| axum::response::IntoResponse::into_response(err))
}

async fn extract_params<T: serde::de::DeserializeOwned + Send>(
    parts: &mut axum::http::request::Parts,
    state: &crate::app::App,
) -> Result<T, axum::response::Response> {
    <axum::extract::Path<T> as axum::extract::FromRequestParts<crate::app::App>>::from_request_parts(parts, state)
        .await
        .map(|v| v.0)
        .map_err(handle_error)
        .map_err(|err| axum::response::IntoResponse::into_response(err))
}


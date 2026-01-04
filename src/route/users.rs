use axum::{extract::Path, response::IntoResponse, Json};
use reqwest::StatusCode;

use crate::{app::App, utils};

pub fn routes(state: App) -> axum::Router {
    use axum::routing::*;
    axum::Router::new()
        .route("/", get(list))
        .route("/{id}", get(info))
        .with_state(state)
}

#[axum::debug_handler]
async fn info(Path(id): Path<uuid::Uuid>) -> impl IntoResponse {
    utils::keycloak::realm_user("sikola", &id.to_string())
        .await
        .map_err(utils::handle_error)?
        .ok_or((StatusCode::NOT_FOUND, "user not found"))
        .map(Json)
}

#[axum::debug_handler]
async fn list() -> impl IntoResponse {
    utils::keycloak::realm_users("sikola", None)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

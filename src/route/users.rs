use axum::{response::IntoResponse, Json};

use crate::{app::App, utils};

pub fn routes(state: App) -> axum::Router {
    use axum::routing::*;
    axum::Router::new().route("/", get(list)).with_state(state)
}

#[axum::debug_handler]
async fn list() -> impl IntoResponse {
    utils::keycloak::realm_users("sikola")
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

use axum::Json;
use axum::extract::State;
use axum::response::IntoResponse;
use sea_orm::{ColumnTrait, EntityTrait, sea_query};

use crate::app::{App, Db};
use crate::model::prelude::*;
use crate::model::*;
use crate::utils;
use crate::utils::jwt::JWTClaims;

pub fn routes(state: App) -> axum::Router {
    use axum::routing::*;
    axum::Router::new()
        .route("/", get(list))
        .with_state(state)
}

#[axum::debug_handler]
async fn list(State(db): State<Db>) -> impl IntoResponse {
    StudentsClasses::find()
        .all(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

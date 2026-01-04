use axum::{
    extract::{FromRequestParts, Path, State},
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::{
    app::{App, Db},
    model, utils,
};

#[derive(Serialize, Deserialize)]
struct ClassParams {
    class_id: uuid::Uuid,
}

impl FromRequestParts<App> for model::classes::Model {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Self, Self::Rejection> {
        let State(db) = <State<Db> as FromRequestParts<App>>::from_request_parts(parts, state)
            .await
            .map_err(utils::handle_error)
            .map_err(|err| err.into_response())?;

        let Path(ClassParams { class_id }) =
            <Path<ClassParams> as FromRequestParts<App>>::from_request_parts(parts, state)
                .await
                .map_err(utils::handle_error)
                .map_err(|err| err.into_response())?;

        model::classes::Entity::find_by_id(class_id)
            .one(&db)
            .await
            .map_err(utils::handle_error)
            .map_err(|err| err.into_response())?
            .ok_or((StatusCode::NOT_FOUND, "class not found").into_response())
    }
}

#[derive(Serialize, Deserialize)]
struct ClassMemberParams {
    class_id: uuid::Uuid,
    user_id: uuid::Uuid,
}

impl FromRequestParts<App> for model::class_memberships::Model {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Self, Self::Rejection> {
        let State(db) = <State<Db> as FromRequestParts<App>>::from_request_parts(parts, state)
            .await
            .map_err(utils::handle_error)
            .map_err(|err| err.into_response())?;

        let Path(params) =
            <Path<ClassMemberParams> as FromRequestParts<App>>::from_request_parts(parts, state)
                .await
                .map_err(utils::handle_error)
                .map_err(|err| err.into_response())?;

        model::class_memberships::Entity::find_by_id((params.class_id, params.user_id))
            .one(&db)
            .await
            .map_err(utils::handle_error)
            .map_err(|err| err.into_response())?
            .ok_or((StatusCode::NOT_FOUND, "class not found").into_response())
    }
}

pub struct ClassWithMemberships(pub (model::classes::Model, Vec<model::class_memberships::Model>));

impl FromRequestParts<App> for ClassWithMemberships {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Self, Self::Rejection> {
        let State(db) = <State<Db> as FromRequestParts<App>>::from_request_parts(parts, state)
            .await
            .map_err(utils::handle_error)
            .map_err(|err| err.into_response())?;

        let Path(params) =
            <Path<ClassParams> as FromRequestParts<App>>::from_request_parts(parts, state)
                .await
                .map_err(utils::handle_error)
                .map_err(|err| err.into_response())?;

        model::classes::Entity::find_by_id(params.class_id)
            .find_with_related(model::class_memberships::Entity)
            .all(&db)
            .await
            .map(|v| v.first().cloned())
            .map_err(utils::handle_error)
            .map_err(|err| err.into_response())?
            .ok_or((StatusCode::NOT_FOUND, "class not found").into_response())
            .map(ClassWithMemberships)
    }
}

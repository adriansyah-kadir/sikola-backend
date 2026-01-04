use axum::{
    extract::FromRequestParts,
    response::{IntoResponse, Response},
};
use reqwest::StatusCode;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

use crate::{
    app::{App, Db},
    model, repos,
    utils::{self, extract_params, extract_state},
};

#[derive(Serialize, Deserialize)]
struct ClassParams {
    class_id: uuid::Uuid,
}

#[derive(Serialize, Deserialize)]
struct ClassMemberParams {
    class_id: uuid::Uuid,
    user_id: uuid::Uuid,
}

pub struct ClassWithMemberships(
    pub model::classes::Model,
    pub Vec<model::class_memberships::Model>,
);
pub struct MembershipWithClass(
    pub model::class_memberships::Model,
    pub Option<model::classes::Model>,
);

impl FromRequestParts<App> for MembershipWithClass {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Self, Self::Rejection> {
        let db: Db = extract_state(parts, state).await?;
        let ClassMemberParams { class_id, user_id } = extract_params(parts, state).await?;

        model::class_memberships::Entity::find_by_id((class_id, user_id))
            .find_also_related(model::classes::Entity)
            .one(&db)
            .await
            .map_err(|e| utils::handle_error(e).into_response())?
            .ok_or((StatusCode::NOT_FOUND, "membership not found").into_response())
            .map(|(m, c)| MembershipWithClass(m, c))
    }
}

impl FromRequestParts<App> for model::classes::Model {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Self, Self::Rejection> {
        let db: Db = extract_state(parts, state).await?;
        let ClassParams { class_id } = extract_params(parts, state).await?;

        model::classes::Entity::find_by_id(class_id)
            .one(&db)
            .await
            .map_err(utils::handle_error)
            .map_err(|err| err.into_response())?
            .ok_or((StatusCode::NOT_FOUND, "class not found").into_response())
    }
}

impl FromRequestParts<App> for model::class_memberships::Model {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Self, Self::Rejection> {
        let db: Db = extract_state(parts, state).await?;
        let ClassMemberParams { class_id, user_id } = extract_params(parts, state).await?;

        model::class_memberships::Entity::find_by_id((class_id, user_id))
            .one(&db)
            .await
            .map_err(utils::handle_error)
            .map_err(|err| err.into_response())?
            .ok_or((StatusCode::NOT_FOUND, "class not found").into_response())
    }
}

impl FromRequestParts<App> for ClassWithMemberships {
    type Rejection = Response;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        state: &App,
    ) -> Result<Self, Self::Rejection> {
        let db: Db = extract_state(parts, state).await?;
        let ClassParams { class_id } = extract_params(parts, state).await?;

        repos::classes::find_by_id_with_related(&db, class_id)
            .await
            .map_err(|err| utils::handle_error(err).into_response())?
            .ok_or((StatusCode::NOT_FOUND, "class not found").into_response())
            .map(|(c, ms)| ClassWithMemberships(c, ms))
    }
}

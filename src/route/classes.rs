use crate::app::{App, Db};
use crate::model::{self, classes_extra};
use crate::repos;
use crate::utils::extractor::MembershipWithClass;
use crate::{
    model::{class_memberships as memberships, classes},
    utils::{self, jwt::JWTClaims},
};
use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
};

pub fn routes(state: crate::app::App) -> axum::Router {
    use axum::routing::*;

    axum::Router::new()
        .route("/", get(class_list))
        .route("/", post(class_create))
        .route("/{class_id}", get(class_info))
        .route("/{class_id}", patch(class_update))
        .route("/{class_id}", delete(class_delete))
        .route("/{class_id}/memberships", get(class_memberships))
        .route("/{class_id}/memberships", post(class_memberships_join))
        .route(
            "/{class_id}/memberships/{user_id}",
            get(class_membership_info),
        )
        .route(
            "/{class_id}/memberships/{user_id}",
            delete(class_memberships_delete),
        )
        .route(
            "/{class_id}/memberships/{user_id}",
            patch(class_memberships_update),
        )
        .route("/available", get(class_available))
        .route("/joined", get(class_joined))
        .with_state(state)
}

async fn class_membership_info(membership: memberships::Model) -> impl IntoResponse {
    Json(membership)
}

async fn class_memberships_update() {
    todo!()
}

async fn class_memberships(
    claims: JWTClaims,
    class: classes::Model,
    State(db): State<Db>,
) -> impl IntoResponse {
    if class.teacher_id != claims.sub {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized"));
    }

    model::prelude::ClassMemberships::find()
        .filter(model::class_memberships::Column::ClassId.eq(class.id))
        .all(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

async fn class_memberships_delete(
    claims: JWTClaims,
    MembershipWithClass(membership, class): MembershipWithClass,
    State(db): State<crate::app::Db>,
) -> impl IntoResponse {
    if class.is_none_or(|c| c.teacher_id != claims.sub) {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized"));
    }

    memberships::Entity::delete(membership.into_active_model())
        .exec_with_returning(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

#[axum::debug_handler(state = App)]
async fn class_joined(claims: JWTClaims, State(db): State<Db>) -> impl IntoResponse {
    classes::Entity::find()
        .inner_join(memberships::Entity)
        .filter(memberships::Column::StudentId.eq(claims.sub))
        .all(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

#[axum::debug_handler(state = App)]
async fn class_memberships_join(
    claims: JWTClaims,
    class: classes::Model,
    State(db): State<Db>,
) -> impl IntoResponse {
    if class.teacher_id == claims.sub {
        return Err((StatusCode::BAD_REQUEST, "teacher cant join"));
    }

    repos::class_members::find_or_insert_membership(db, claims.sub, class.id)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

#[axum::debug_handler(state = App)]
async fn class_available(claims: JWTClaims, State(db): State<crate::app::Db>) -> impl IntoResponse {
    repos::classes::available(db, claims.sub)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

#[axum::debug_handler(state = crate::app::App)]
async fn class_delete(
    claims: JWTClaims,
    class: classes::Model,
    State(db): State<crate::app::Db>,
) -> impl IntoResponse {
    if class.teacher_id != claims.sub {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized"));
    }

    classes::Entity::delete(class.into_active_model())
        .exec_with_returning(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

#[axum::debug_handler(state = crate::app::App)]
async fn class_update(
    claims: JWTClaims,
    class: classes::Model,
    State(db): State<crate::app::Db>,
    Json(body): Json<classes_extra::ClassRequiredBody>,
) -> impl IntoResponse {
    if class.teacher_id != claims.sub {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized"));
    }

    if repos::classes::can_use_name(&db, class.id, &body.name)
        .await
        .map_err(utils::handle_error)?
    {
        return Err((StatusCode::CONFLICT, "name is used"));
    }

    classes::ActiveModel {
        id: sea_orm::Set(class.id),
        ..body.into_active_model()
    }
    .update(&db)
    .await
    .map(Json)
    .map_err(utils::handle_error)
}

#[axum::debug_handler(state = crate::app::App)]
async fn class_info(class: classes::Model) -> impl IntoResponse {
    Json(class)
}

#[axum::debug_handler(state = crate::app::App)]
async fn class_create(
    claims: JWTClaims,
    State(db): State<crate::app::Db>,
    Json(body): Json<classes_extra::ClassRequiredBody>,
) -> impl axum::response::IntoResponse {
    if classes::Entity::find_by_name(&body.name)
        .exists(&db)
        .await
        .map_err(utils::handle_error)?
    {
        return Err((StatusCode::CONFLICT, "name already used"));
    }

    classes::ActiveModel {
        name: sea_orm::Set(body.name),
        description: sea_orm::Set(body.description),
        teacher_id: sea_orm::Set(claims.sub),
        ..Default::default()
    }
    .insert(&db)
    .await
    .map(Json)
    .map_err(utils::handle_error)
}

#[axum::debug_handler(state = crate::app::App)]
async fn class_list(State(db): State<crate::app::Db>) -> impl axum::response::IntoResponse {
    classes::Entity::find()
        .all(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

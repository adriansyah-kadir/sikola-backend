use crate::{
    model::classes,
    utils::{self, jwt::JWTClaims},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, IntoActiveModel, PaginatorTrait, QueryFilter,
};

pub fn routes(state: crate::app::App) -> axum::Router {
    use axum::routing::*;

    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/{id}", get(info))
        .route("/{id}", post(update))
        .route("/{id}", delete(remove))
        .with_state(state)
}

#[axum::debug_handler(state = crate::app::App)]
async fn remove(
    claims: JWTClaims,
    Path(id): Path<uuid::Uuid>,
    State(db): State<crate::app::Db>,
) -> impl IntoResponse {
    let class = classes::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(utils::handle_error)?
        .ok_or((StatusCode::NOT_FOUND, "not found"))?;

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
async fn update(
    claims: JWTClaims,
    Path(id): Path<uuid::Uuid>,
    State(db): State<crate::app::Db>,
    Json(body): Json<classes::Required>,
) -> impl IntoResponse {
    let class = classes::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(utils::handle_error)?
        .ok_or((StatusCode::NOT_FOUND, "not found"))?;

    if class.teacher_id != claims.sub {
        return Err((StatusCode::UNAUTHORIZED, "unauthorized"));
    }

    if class.name == body.name {
        return Err((StatusCode::NOT_MODIFIED, "no change"));
    }

    if classes::Entity::find_by_name(&body.name)
        .filter(classes::Column::Id.ne(id))
        .exists(&db)
        .await
        .map_err(utils::handle_error)?
    {
        return Err((StatusCode::CONFLICT, "name is used"));
    }

    let mut model = body.into_active_model();
    model.id = sea_orm::Set(id);

    model
        .update(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

#[axum::debug_handler(state = crate::app::App)]
async fn info(Path(id): Path<uuid::Uuid>, State(db): State<crate::app::Db>) -> impl IntoResponse {
    classes::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(utils::handle_error)?
        .map(Json)
        .ok_or((StatusCode::NOT_FOUND, "not found"))
}

#[axum::debug_handler(state = crate::app::App)]
async fn create(
    State(db): State<crate::app::Db>,
    claims: JWTClaims,
    Json(body): Json<classes::Required>,
) -> impl axum::response::IntoResponse {
    use sea_orm::ActiveValue;

    if classes::Entity::find_by_name(body.name.clone())
        .exists(&db)
        .await
        .map_err(utils::handle_error)?
    {
        return Err((StatusCode::CONFLICT, "name already used"));
    }

    let insert = classes::ActiveModel {
        name: ActiveValue::Set(body.name),
        description: ActiveValue::Set(body.description),
        teacher_id: ActiveValue::Set(claims.sub),
        ..Default::default()
    };

    insert
        .insert(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

#[axum::debug_handler]
async fn list(State(db): State<crate::app::Db>) -> impl axum::response::IntoResponse {
    classes::Entity::find()
        .all(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
}

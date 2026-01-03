use crate::app::{App, Db};
use crate::{
    model::{classes, students_classes},
    utils::{self, jwt::JWTClaims},
};
use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, ExprTrait, IntoActiveModel, PaginatorTrait,
    QueryFilter, QuerySelect, RelationTrait, prelude::Expr, sea_query::IntoCondition,
};

pub fn routes(state: crate::app::App) -> axum::Router {
    use axum::routing::*;

    axum::Router::new()
        .route("/", get(list))
        .route("/", post(create))
        .route("/{id}", get(info))
        .route("/{id}", post(update))
        .route("/{id}", delete(remove))
        .route("/{id}/join", post(join))
        .route("/available", get(available))
        .with_state(state)
}

#[axum::debug_handler(state = App)]
async fn join(
    claims: JWTClaims,
    State(db): State<Db>,
    Path(id): Path<uuid::Uuid>,
) -> impl IntoResponse {
    let class = classes::Entity::find_by_id(id)
        .one(&db)
        .await
        .map_err(utils::handle_error)?
        .ok_or((StatusCode::NOT_FOUND, "not found"))?;

    if class.teacher_id == claims.sub {
        return Err((StatusCode::BAD_REQUEST, "teacher cant join"));
    }

    if let Some(student_classes) = students_classes::Entity::find_by_id((claims.sub, id))
        .one(&db)
        .await
        .map_err(utils::handle_error)?
    {
        return Ok(Json(student_classes));
    } else {
        students_classes::Entity::insert(students_classes::ActiveModel {
            student_id: sea_orm::Set(claims.sub),
            class_id: sea_orm::Set(id),
            ..Default::default()
        })
        .exec_with_returning(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
    }
}

#[axum::debug_handler(state = App)]
async fn available(claims: JWTClaims, State(db): State<crate::app::Db>) -> impl IntoResponse {
    classes::Entity::find()
        .join(
            sea_orm::JoinType::LeftJoin,
            classes::Relation::StudentsClasses
                .def()
                .on_condition(move |_, r| {
                    Expr::col((r, students_classes::Column::StudentId))
                        .eq(claims.sub)
                        .into_condition()
                }),
        )
        .filter(students_classes::Column::ClassId.is_null())
        .filter(classes::Column::TeacherId.ne(claims.sub))
        .all(&db)
        .await
        .map(Json)
        .map_err(utils::handle_error)
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

#[axum::debug_handler(state = crate::app::App)]
async fn list(
    claims: Option<JWTClaims>,
    State(db): State<crate::app::Db>,
) -> impl axum::response::IntoResponse {
    match claims {
        Some(claims) => classes::Entity::find()
            .join(
                sea_orm::JoinType::LeftJoin,
                students_classes::Relation::Classes
                    .def()
                    .rev()
                    .on_condition(move |_, r| {
                        Expr::col((r, students_classes::Column::StudentId))
                            .eq(claims.sub)
                            .into_condition()
                    }),
            )
            .filter(students_classes::Column::ClassId.is_null())
            .all(&db)
            .await
            .map(Json)
            .map_err(utils::handle_error),
        None => classes::Entity::find()
            .all(&db)
            .await
            .map(Json)
            .map_err(utils::handle_error),
    }
}

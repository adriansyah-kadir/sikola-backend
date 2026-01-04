use sea_orm::sea_query::IntoCondition;
use sea_orm::{EntityTrait, QuerySelect, RelationTrait};
use sea_orm::{ExprTrait, prelude::*};

use crate::app::Db;
use crate::model::prelude::*;
use crate::model::*;

pub async fn available(db: Db, user_id: uuid::Uuid) -> Result<Vec<classes::Model>, sea_orm::DbErr> {
    Classes::find()
        .join(
            sea_orm::JoinType::LeftJoin,
            classes::Relation::ClassMembers
                .def()
                .on_condition(move |_, r| {
                    Expr::col((r, class_members::Column::StudentId))
                        .eq(user_id)
                        .into_condition()
                }),
        )
        .filter(class_members::Column::ClassId.is_null())
        .filter(classes::Column::TeacherId.ne(user_id))
        .all(&db)
        .await
}

pub async fn can_use_name(db: &Db, class_id: uuid::Uuid, name: &str) -> Result<bool, DbErr> {
    classes::Entity::find_by_name(name)
        .filter(classes::Column::Id.ne(class_id))
        .exists(db)
        .await
}

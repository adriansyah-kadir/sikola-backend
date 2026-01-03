use sea_orm::{prelude::*, ExprTrait};
use sea_orm::sea_query::IntoCondition;
use sea_orm::{EntityTrait, QuerySelect, RelationTrait};

use crate::app::Db;
use crate::model::prelude::*;
use crate::model::*;

pub async fn available(
    db: Db,
    user_id: uuid::Uuid,
) -> Result<Vec<classes::Model>, sea_orm::DbErr> {
    Classes::find()
        .join(
            sea_orm::JoinType::LeftJoin,
            classes::Relation::StudentsClasses
                .def()
                .on_condition(move |_, r| {
                    Expr::col((r, students_classes::Column::StudentId)).eq(user_id).into_condition()
                }),
        )
        .filter(students_classes::Column::ClassId.is_null())
        .filter(classes::Column::TeacherId.ne(user_id))
        .all(&db)
        .await
}

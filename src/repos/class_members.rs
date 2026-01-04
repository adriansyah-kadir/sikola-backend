use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::{app::Db, model, model::prelude::*};

pub async fn find_or_insert_membership(
    db: Db,
    student_id: uuid::Uuid,
    class_id: uuid::Uuid,
) -> Result<model::class_memberships::Model, sea_orm::error::DbErr> {
    let existing = ClassMemberships::find_by_id((student_id, class_id))
        .one(&db)
        .await?;

    if let Some(membership) = existing {
        Ok(membership)
    } else {
        model::class_memberships::ActiveModel {
            class_id: sea_orm::Set(class_id),
            student_id: sea_orm::Set(student_id),
            ..Default::default()
        }
        .insert(&db)
        .await
    }
}

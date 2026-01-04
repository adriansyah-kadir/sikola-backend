use sea_orm::{ActiveModelTrait, EntityTrait};

use crate::{app::Db, model::class_members};

pub async fn find_or_insert_membership(
    db: Db,
    student_id: uuid::Uuid,
    class_id: uuid::Uuid,
) -> Result<class_members::Model, sea_orm::error::DbErr> {
    let existing = class_members::Entity::find_by_id((student_id, class_id))
        .one(&db)
        .await?;

    if let Some(membership) = existing {
        Ok(membership)
    } else {
        class_members::ActiveModel {
            class_id: sea_orm::Set(class_id),
            student_id: sea_orm::Set(student_id),
            ..Default::default()
        }
        .insert(&db)
        .await
    }
}

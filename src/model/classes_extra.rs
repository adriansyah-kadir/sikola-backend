use crate::model::classes::*;
use sea_orm::DeriveIntoActiveModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, DeriveIntoActiveModel)]
pub struct ClassRequiredBody {
    pub name: String,
    pub description: Option<String>
}

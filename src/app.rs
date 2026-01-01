pub type Db = sea_orm::DatabaseConnection;

#[derive(Clone, axum::extract::FromRef)]
pub struct App {
    pub db: Db
}

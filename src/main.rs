use tower_http::request_id::{self, SetRequestIdLayer};

mod app;
mod model;
mod route;
mod utils;
mod repos;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let db = dotenvy::var("DATABASE_URL")
        .map(|db_url| sea_orm::Database::connect(db_url))
        .expect("failed to get database url")
        .await
        .expect("database connection failed");

    let state = app::App { db };

    let middleware = tower::ServiceBuilder::new()
        .layer(SetRequestIdLayer::x_request_id(request_id::MakeRequestUuid))
        .layer(request_id::PropagateRequestIdLayer::x_request_id())
        .layer(utils::middleware::make_trace_http_layer())
        .layer(tower_http::cors::CorsLayer::permissive());

    let service = axum::Router::new()
        .nest("/classes", route::classes::routes(state.clone()))
        .nest("/students_classes", route::students_classes::routes(state.clone()))
        .layer(middleware);

    let listener = tokio::net::TcpListener::bind("localhost:3000")
        .await
        .expect("failed to bind address");

    axum::serve(listener, service)
        .await
        .expect("failed to run server");
}

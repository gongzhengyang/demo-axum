use std::net::SocketAddr;
use std::str::FromStr;
use std::sync::Arc;

use aide::{axum::ApiRouter, openapi::OpenApi};
use axum::{
    Extension,
    handler::HandlerWithoutStateExt,
    routing::{get},
    Server
};
use tower::ServiceBuilder;

use tower_http::cors::{Any, CorsLayer};

use db::get_db_connection;
pub use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let rust_log_key = "RUST_LOG";
    let rust_log_level = std::env::var(rust_log_key)
        .unwrap_or_else(|_| {
            std::env::set_var(rust_log_key, "INFO");
            std::env::var(rust_log_key).unwrap()
        });
    println!("rust log is: {rust_log_level}");
    tracing_subscriber::fmt::init();
    let db = get_db_connection().await;
    Migrator::up(db, None).await?;

    aide::gen::on_error(|error| {
        println!("{error}");
    });

    aide::gen::extract_schemas(true);
    let mut api = OpenApi::default();
    let router = ApiRouter::new()
        .nest_api_service("/model", api::post_router())
        .nest_api_service("/docs", api::docs::api_docs_json())
        .route("/statics/index.html", get(api::statics::index_handler))
        .route_service("/statics/*file", api::statics::static_handler.into_service())
        .finish_api_with(&mut api, api::docs::api_docs)
        .layer(
            ServiceBuilder::new()
                .layer(Extension(db))
                .layer(Extension(Arc::new(api)))
                .layer(CorsLayer::new().allow_origin(Any)),
        );

    let addr = SocketAddr::from_str(get_server_url().as_str()).expect("host:port is error");
    tracing::info!("listen {:?}", addr);
    Server::bind(&addr)
        .serve(router.into_make_service())
        .await?;
    Ok(())
}

fn get_server_url() -> String {
    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("SERVER_PORT").unwrap_or_else(|_| "8088".into());
    format!("{host}:{port}")
}

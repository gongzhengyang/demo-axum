use std::net::SocketAddr;
use std::str::FromStr;

use axum::{Extension, Router, routing::{post, put}, Server};
use tokio;
use tower::ServiceBuilder;
use tracing;

use api;
use db::get_db_connection;
pub use migration::{Migrator, MigratorTrait};

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    let db = get_db_connection().await;
    Migrator::up(db, None).await?;

    let addr = SocketAddr::from_str(get_server_url().as_str())
        .expect("host:port is error");
    tracing::info!("listen {:?}", addr);
    Server::bind(&addr)
        .serve(Router::new()
            .nest("/model",
                  Router::new()
                      .route("/", post(api::create).get(api::list))
                      .route("/:id", put(api::update).delete(api::delete)))
            .layer(
                ServiceBuilder::new()
                    .layer(Extension(db)),
            ).into_make_service())
        .await?;
    Ok(())
}

fn get_server_url() -> String {
    let host = std::env::var("SERVER_HOST").unwrap_or("0.0.0.0".into());
    let port = std::env::var("SERVER_PORT").unwrap_or("8088".into());
    format!("{}:{}", host, port)
}

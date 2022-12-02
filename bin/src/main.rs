use db::get_db_connection;
pub use migration::{Migrator, MigratorTrait};
use axum::{Router};
use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = get_db_connection().await;
    Migrator::up(db, None).await?;


    // let server_url
    Ok(())
}

fn get_server_url() -> String {
    let host = std::env::var("SERVER_HOST", "0.0.0.0");
    let port = env!("SERVER_PORT", "8088");
    format!("{}:{}", host, port)
}

// fn get_app_router() {
//     Router::new()
// }
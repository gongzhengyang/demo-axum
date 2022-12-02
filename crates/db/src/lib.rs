use std::env;
use std::time::Duration;

use tokio::sync::OnceCell;
use once_cell::sync::Lazy;
use sea_orm::{entity::prelude::DatabaseConnection, ConnectOptions, Database};

pub static DB_CONNECT: Lazy<OnceCell<DatabaseConnection>> = Lazy::new(OnceCell::new);

pub async fn db_connect() -> DatabaseConnection {
    let db_name = env::var("POSTGRES_DB").unwrap_or("demo".into());
    let db_user = env::var("POSTGRES_USER").unwrap_or("demo-user".into());
    let db_password = env::var("POSTGRES_PASSWORD").unwrap_or("demo-password".into());
    let db_host = env::var("POSTGRES_HOST").unwrap_or("127.0.0.1".into());
    let db_port = env::var("POSTGRES_PORT").unwrap_or("5432".into());

    let mut options = ConnectOptions::new(
        env::var("DB_URL")
            .unwrap_or(
                format!("postgres://{}:{}@{}:{}/{}",
                        db_user, db_password, db_host, db_port, db_name)));
    options.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .set_schema_search_path("public".into()); // Setting default PostgreSQL schema

    let db = Database::connect(options)
        .await.expect("database connect failed");
    tracing::info!("Database connected !");
    db
}

pub async fn get_db_connection() -> &'static DatabaseConnection{
    DB_CONNECT.get_or_init(db_connect).await
}

use db::get_db_connection;
pub use migration::{Migrator, MigratorTrait};

use tokio;

#[tokio::main]
async fn main() -> anyhow::Result<()>{
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();

    let db = get_db_connection().await;
    Migrator::up(db, None).await?;

    Ok(())
}

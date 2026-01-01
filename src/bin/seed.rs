use sea_orm::Database;
use server_test_csharp::{config, mock};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = config::config::Config::init();
    let db = Database::connect(config.database_url).await?;

    println!("Connected to database.");
    println!("Performing seeding");
    mock::seed::seed_all(&db).await?;

    println!("Seeding completed.");
    Ok(())
}
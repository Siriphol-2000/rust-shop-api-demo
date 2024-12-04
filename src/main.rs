use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables
    dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL")?;

    // Establish a connection to the database
    let db: DatabaseConnection = Database::connect(&database_url).await?;

    println!("Connected to the database!");

    Ok(())
}

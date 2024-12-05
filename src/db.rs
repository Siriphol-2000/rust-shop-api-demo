use sea_orm::{Database, DatabaseConnection, DbErr};
use dotenvy::dotenv;
use std::env;
 
// Function to establish a connection to the database
pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    dotenv().ok(); // Load environment variables from `.env` file

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Connect to the database using SeaORM and return Result
    Database::connect(&database_url).await
}

pub async fn get_database_connection() -> Result<DatabaseConnection, DbErr> {
    // Establish connection and return as Result
    Ok(establish_connection().await?)
}

use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env;

// Function to establish a connection to the database
pub async fn establish_connection() -> DatabaseConnection {
    dotenv().ok(); // Load environment variables

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");

    // Connect to the database using SeaORM
    Database::connect(&database_url)
        .await
        .expect("Failed to connect to the database")
}

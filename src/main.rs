use actix_web::{web, App, HttpServer, Responder, HttpResponse};
use sea_orm::{Database, DatabaseConnection};
use dotenvy::dotenv;
use std::env;

mod db;  // Module for database connection
mod models;  // Module for SeaORM models

// Default route handler function
async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file
    dotenv().ok();

    // Get the database URL from the environment variable
    let database_url = env::var("DATABASE_URL")?;

    // Establish a connection to the database
    let db: DatabaseConnection = Database::connect(&database_url).await?;

    println!("Connected to the database!");

    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))  // Pass the DB connection to Actix
            .route("/", web::get().to(default_route)) // Default route handler
    })
    .bind("127.0.0.1:8080")?  // Bind server to address
    .run()
    .await?;

    Ok(())
}

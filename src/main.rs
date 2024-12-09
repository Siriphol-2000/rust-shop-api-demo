use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use db::establish_connection;
use handler::product_handler::*;
use handler::user_handler::*;
use tracing_subscriber; // Import for tracing setup
use tracing::Level; // Log level
mod db; // Module for database connection
pub mod entities;
mod models; // Module for SeaORM models
mod services;
mod utils;
mod handler;

// Default route handler function
async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let db = establish_connection().await?;

    println!("Connected to the database!");
    
// Setup tracing for logging
tracing_subscriber::fmt()
.with_max_level(Level::DEBUG)  // Set log level to DEBUG
.init();  // Initialize the subscriber

    // Start Actix Web server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone())) // Pass the DB connection to Actix
            .route("/", web::get().to(default_route)) // Default route handler
            .service(register) // Add your register route to the app
            .service(get_user) // Add the GET route
            .service(login) // Add the login route
            .service(create_product)
            .service(get_product)
            .service(update_product)
            .service(delete_product)
            .service(get_all_products)
    })
    .bind("127.0.0.1:8080")? // Bind server to address
    .run()
    .await?;

    Ok(())
}

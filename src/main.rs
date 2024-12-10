use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use db::establish_connection;
use handler::cart_handler::*;
use handler::order_handler::*;
use handler::product_handler::*;
use handler::user_handler::*;
use tracing::Level;

mod db; // Module for database connection
pub mod entities;
mod handler;
mod models; // Module for SeaORM models
mod services;
mod utils;

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
        .with_max_level(Level::DEBUG) // Set log level to DEBUG
        .init(); // Initialize the subscriber

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
            .service(create_cart_handler)
            .service(add_item_to_cart_handler)
            .service(remove_item_from_cart_handler)
            .service(clear_cart_handler)
            .service(create_order_handler)
            .service(get_order_with_items_handler)
            .service(update_payment_status_handler)
            .service(delete_order_handler)
            .service(create_order_handler) //orders
            .service(get_order_with_items_handler)
            .service(update_payment_status_handler)
            .service(delete_order_handler)
    })
    .bind("127.0.0.1:8080")? // Bind server to address
    .run()
    .await?;

    Ok(())
}

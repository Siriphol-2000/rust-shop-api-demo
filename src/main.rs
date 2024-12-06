use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use db::establish_connection;
use dotenvy::dotenv;
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub mod entities;
use entities::{prelude::*, *};

mod routes;
use routes::user_routes::*;
use routes::product_routes::*;

mod db; // Module for database connection
mod models; // Module for SeaORM models
mod services;

// Default route handler function
async fn default_route() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // // Load environment variables from .env file
    // dotenv().ok();

    // // Get the database URL from the environment variable
    // let database_url = env::var("DATABASE_URL")?;

    // // Establish a connection to the database
    // let db: DatabaseConnection = Database::connect(&database_url).await?;
    let db = establish_connection().await?;

    println!("Connected to the database!");

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

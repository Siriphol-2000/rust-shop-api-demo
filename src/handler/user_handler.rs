use crate::models::user::{UserLoginRequest, UserRegisterRequest};
use crate::services::user_service;
use actix_web::{get, post, web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[post("/register")]
async fn register(
    db: web::Data<DatabaseConnection>,
    request: web::Json<UserRegisterRequest>,
) -> impl Responder {
    // Validate the input
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(validation_errors);
    }

    // Call the service
    match user_service::register_user(db.get_ref(), request.into_inner()).await {
        Ok(user_response) => HttpResponse::Created().json(user_response),
        Err(err) => HttpResponse::InternalServerError().body(err),
    }
}

#[get("/users/{id}")]
async fn get_user(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>, // Extract `id` from the URL
) -> impl Responder {
    match user_service::get_user_by_id(db.get_ref(), *user_id).await {
        Ok(user_response) => HttpResponse::Ok().json(user_response),
        Err(err) => HttpResponse::NotFound().body(err),
    }
}

#[post("/login")]
async fn login(
    db: web::Data<DatabaseConnection>,
    request: web::Json<UserLoginRequest>,
) -> impl Responder {
    // Validate the input
    if let Err(validation_errors) = request.validate() {
        return HttpResponse::BadRequest().json(validation_errors);
    }

    // Authenticate the user
    match user_service::authenticate_user(db.get_ref(), &request.email, &request.password).await {
        Ok(token) => HttpResponse::Ok().json(token), // Return a token or session info
        Err(err) => HttpResponse::Unauthorized().body(err),
    }
}

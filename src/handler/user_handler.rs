use crate::models::user::{UserLoginRequest, UserRegisterRequest};
use crate::services::user_service;
use crate::utils::actix_error::ApiError;
use crate::ApiResponse;
use actix_web::{get, post, web, HttpResponse};
use sea_orm::DatabaseConnection;
use validator::Validate;

#[post("/register")]
async fn register(
    db: web::Data<DatabaseConnection>,
    request: web::Json<UserRegisterRequest>,
) -> Result<HttpResponse, ApiError> {
    request.validate()?;

    let user_response = user_service::register_user(db.get_ref(), request.into_inner()).await?;
    Ok(HttpResponse::Created().json(ApiResponse {
        status: "success".to_string(),
        message: "registration successfully".to_string(),
        data: Some(user_response),
    }))
}

#[get("/users/{id}")]
async fn get_user(
    db: web::Data<DatabaseConnection>,
    user_id: web::Path<i32>, // Extract `id` from the URL
) -> Result<HttpResponse, ApiError> {
    let user_response = user_service::get_user_by_id(db.get_ref(), *user_id).await?;
    Ok(HttpResponse::Ok().json(ApiResponse {
        status: "success".to_string(),
        message: "Product fetched successfully".to_string(),
        data: Some(user_response),
    }))
}

#[post("/login")]
async fn login(
    db: web::Data<DatabaseConnection>,
    request: web::Json<UserLoginRequest>,
) -> Result<HttpResponse, ApiError> {
    // Validate the input
    request.validate()?;

    let user_response =
        user_service::authenticate_user(db.get_ref(), &request.email, &request.password).await?;

    Ok(HttpResponse::Created().json(ApiResponse {
        status: "success".to_string(),
        message: "login successfully".to_string(),
        data: Some(user_response),
    }))
}

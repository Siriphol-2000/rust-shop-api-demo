use actix_web::{post, web, HttpResponse, Responder};
use crate::models::user::UserRegisterRequest;
use crate::services::user_service;
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

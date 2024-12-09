use crate::entities::user;
use crate::models::user::{UserLoginResponse, UserRegisterRequest, UserResponse};
use crate::utils::jwt::generate_jwt;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, Set};

pub async fn register_user(
    db: &DatabaseConnection,
    request: UserRegisterRequest,
) -> Result<UserResponse, String> {
    // Generate a random salt for password hashing
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash the password using Argon2
    let password_hash = argon2
        .hash_password(request.password.as_bytes(), &salt)
        .map_err(|_| "Failed to hash password")?
        .to_string(); // Get the password hash as a string

    // Create a new user record
    let new_user = user::ActiveModel {
        email: Set(request.email.clone()),
        password_hash: Set(password_hash),
        ..Default::default()
    };

    // Insert the user into the database
    let result = new_user
        .insert(db)
        .await
        .map_err(|_| "Failed to save user")?;

    // Return the response with user details
    Ok(UserResponse {
        id: result.id,
        email: result.email,
    })
}
pub async fn get_user_by_id(db: &DatabaseConnection, user_id: i32) -> Result<UserResponse, String> {
    // Fetch the user by ID
    match user::Entity::find_by_id(user_id).one(db).await {
        Ok(Some(user)) => Ok(UserResponse {
            id: user.id,
            email: user.email,
        }),
        Ok(None) => Err(format!("User with ID {} not found", user_id)),
        Err(err) => Err(format!("Failed to fetch user: {}", err)),
    }
}

pub async fn authenticate_user(
    db: &DatabaseConnection,
    email: &str,
    password: &str,
) -> Result<UserLoginResponse, String> {
    // Fetch user by email
    let user = user::Entity::find()
        .filter(user::Column::Email.eq(email))
        .one(db)
        .await
        .map_err(|_| "Database error")?
        .ok_or("User not found")?;

    // Verify password
    let parsed_hash =
        PasswordHash::new(&user.password_hash).map_err(|_| "Invalid password hash")?;
    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_err()
    {
        return Err("Invalid credentials".to_string());
    }

    // Clone user.email to avoid moving it
    let token = generate_jwt(user.id, user.email.clone()).map_err(|_| "Failed to generate JWT")?;

    // Return success with token
    Ok(UserLoginResponse {
        id: user.id,
        email: user.email,
        token,
    })
}

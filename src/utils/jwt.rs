use chrono::Utc;
use jsonwebtoken::{encode, errors::Error, EncodingKey, Header};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: i32, // User ID
    pub email: String,
    pub exp: usize, // Expiration time
}

pub fn generate_jwt(user_id: i32, email: String) -> Result<String, Error> {
    let expiration = 3600; // Token expiration time in seconds (1 hour)

    // Set claims
    let claims = Claims {
        sub: user_id,
        email,
        exp: (Utc::now().timestamp() + expiration as i64) as usize,
    };

    // Secret key (should be stored in env variables in production)
    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");

    // Create JWT token
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )?;

    Ok(token)
}

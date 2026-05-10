use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};

use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use argon2::password_hash::rand_core::OsRng;
use chrono::{Utc, Duration};
use anyhow::Result;

pub fn hash_password(plain: &str) -> Result<String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(plain.as_bytes(), &salt)
        .map_err(|e| anyhow::anyhow!("Password hashing failed: {}", e))?
        .to_string();
    Ok(hash)
}

pub fn verify_password(plain: &str, hash: &str) -> Result<bool> {
    let parsed = PasswordHash::new(hash)
        .map_err(|e| anyhow::anyhow!("Failed to parse password hash: {}", e))?;
    let argon2 = Argon2::default();
    Ok(argon2.verify_password(plain.as_bytes(), &parsed).is_ok())
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,   
    iat: i64,      
    exp: i64,      
}

pub fn generate_jwt(secret: &str, user_id: Uuid) -> Result<String> {
    let now = Utc::now();
    let exp = now + Duration::minutes(15);

    let claims = Claims {
        sub: user_id.to_string(),
        iat: now.timestamp(),
        exp: exp.timestamp(),
    };
    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )?;
    Ok(token)
}

#[allow(dead_code)]
pub fn verify_jwt(secret: &str, token: &str) -> Result<Uuid> {
    let data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )?;
    let uid = Uuid::parse_str(&data.claims.sub)?;
    Ok(uid)
}
// API запросы/ответы:

use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,

    #[serde(skip_serializing)]
    pub password_hash: String,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { success: true, data: Some(data), error: None }
    }
    pub fn error(msg: String) -> Self {
        Self { success: false, data: None, error: Some(msg) }
    }
} 

// Конфигурация приложения:

use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub database: String,
    pub username: String,
    pub password: String,
    #[serde(default = "default_pool")]
    pub pool_size: u32,
}
fn default_pool() -> u32 { 10 }

#[derive(Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    #[serde(default)]
    pub cors_origins: Vec<String>,
    #[serde(default = "default_timeout")]
    pub request_timeout: u64,
}
fn default_timeout() -> u64 { 30 }

#[derive(Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub redis: Option<RedisConfig>,
    pub features: HashMap<String, bool>,
}

#[derive(Deserialize)]
pub struct RedisConfig { pub url: String } 

// Валидация на входе через кастомные десериализаторы:

use serde::{Serialize, Deserialize, Deserializer};
use serde::de::{self, Error as _};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: u32,
    pub name: String,
    #[serde(deserialize_with = "validate_email")]
    pub email: String,
    #[serde(deserialize_with = "validate_password")]
    pub password: String,
}

fn validate_email<'de, D>(d: D) -> Result<String, D::Error>
where D: Deserializer<'de> {
    let email = String::deserialize(d)?;
    if email.contains('@') && email.len() > 5 { Ok(email) }
    else { Err(de::Error::custom("Invalid email format")) }
}

fn validate_password<'de, D>(d: D) -> Result<String, D::Error>
where D: Deserializer<'de> {
    let p = String::deserialize(d)?;
    if p.len() >= 8 { Ok(p) }
    else { Err(de::Error::custom("Password must be at least 8 characters")) }
} 

// Кеширование (Redis + JSON):

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CachedUser {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub cached_at: chrono::DateTime<chrono::Utc>,
} 
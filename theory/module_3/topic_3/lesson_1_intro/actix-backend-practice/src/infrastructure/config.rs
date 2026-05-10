// Здесь прописываются данные из .env-файла, чтобы проект мог использовать их, не показывая секреты в коде.

use serde::Deserialize;
use anyhow::Context;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub jwt_secret: String,
    pub cors_origin: String,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        let database_url = std::env::var("DATABASE_URL")
            .context("DATABASE_URL must be set in .env")?;
        
        let host = std::env::var("HOST")
            .unwrap_or_else(|_| "127.0.0.1".into());
        
        let port = std::env::var("PORT")
            .unwrap_or_else(|_| "8080".into())
            .parse()
            .context("PORT must be a valid number")?;
        
        let jwt_secret = std::env::var("JWT_SECRET")
            .context("JWT_SECRET must be set in .env")?;
        
        // 🔥 CHANGE THIS LINE:
        let cors_origin = std::env::var("CORS_ORIGIN")
            .context("CORS_ORIGIN must be set in .env file")?;  // ← No wildcard fallback!
            // .unwrap_or_else(|_| "*".into());  // ← Delete this fallback

        Ok(Self {
            database_url,
            host,
            port,
            jwt_secret,
            cors_origin: cors_origin.trim().to_string(),  // ← Trim whitespace
        })
    }
} 
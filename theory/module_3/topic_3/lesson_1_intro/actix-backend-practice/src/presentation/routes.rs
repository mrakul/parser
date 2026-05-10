use actix_web::{get, post, web, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

use crate::data::users_repo;
use crate::infrastructure::{security, config::Config};

#[derive(Deserialize)]
struct RegisterDto {
    email: String,
    password: String,
}

#[derive(Deserialize)]
struct LoginDto {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct TokenResponse {
    access_token: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status":"ok"}))
}

#[post("/register")]
async fn register(
    pool: web::Data<PgPool>,
    body: web::Json<RegisterDto>,
) -> actix_web::Result<impl Responder> {
    let email = body.email.trim().to_lowercase();
    if email.is_empty() || body.password.len() < 6 {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": "invalid input"
        })));
    }

    let pw_hash = security::hash_password(&body.password)
        .map_err(|_| actix_web::error::ErrorInternalServerError("hash error"))?;

    let user_id = uuid::Uuid::new_v4();

    let res = users_repo::create_user(&pool, user_id, &email, &pw_hash).await;
    
    match res {
        Ok(_) => Ok(HttpResponse::Created().finish()),
        Err(sqlx::Error::Database(db_err)) if db_err.is_unique_violation() => {
            Ok(HttpResponse::Conflict().json(serde_json::json!({"error":"email taken"})))
        }
        Err(_) => Err(actix_web::error::ErrorInternalServerError("db error")),
    }
}

#[post("/login")]
async fn login(
    pool: web::Data<PgPool>,
    cfg: web::Data<Config>,
    body: web::Json<LoginDto>,
) -> actix_web::Result<impl Responder> {
    let email = body.email.trim().to_lowercase();

    let user = match users_repo::find_by_email(&pool, &email).await {
        Ok(Some(u)) => u,
        Ok(None) => return Ok(HttpResponse::Unauthorized().finish()),
        Err(_) => return Err(actix_web::error::ErrorInternalServerError("db error")),
    };

    let ok = security::verify_password(&body.password, &user.password_hash)
        .map_err(|_| actix_web::error::ErrorInternalServerError("verify error"))?;

    if !ok {
        return Ok(HttpResponse::Unauthorized().finish());
    }

    let token = security::generate_jwt(&cfg.jwt_secret, user.id)
        .map_err(|_| actix_web::error::ErrorInternalServerError("jwt error"))?;

    Ok(HttpResponse::Ok().json(TokenResponse { access_token: token }))
}

pub fn configure(cfg: &mut actix_web::web::ServiceConfig) {
    cfg.service(health)
        .service(register)
        .service(login);
}
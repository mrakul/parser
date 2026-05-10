mod presentation;
mod application;
mod domain;
mod data;
mod infrastructure;

use actix_cors::Cors;
use actix_web::{middleware::Logger, web, App, HttpServer};
use infrastructure::{config::Config, migrate};
use sqlx::postgres::PgPoolOptions;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::init();

    // 🔥 Explicitly load .env from project root
    // use std::env;
    // let manifest_dir = env!("CARGO_MANIFEST_DIR");
    // let env_path = format!("{}/.env", manifest_dir);
    
    // match dotenvy::from_path(&env_path) {
    //     Ok(_) => eprintln!("✅ Loaded .env from: {}", env_path),
    //     Err(e) => eprintln!("❌ Failed to load .env: {}", e),
    // }

    // Debug: show what's in the environment AFTER dotenv
    eprintln!("🔍 [AFTER dotenv] CORS_ORIGIN = {:?}", std::env::var("CORS_ORIGIN").ok());


    let cfg = Config::from_env().expect("invalid config");
    
    // Debug: show parsed config value
    eprintln!("🔍 [PARSED CONFIG] cors_origin = '{:?}'", cfg.cors_origin);
    eprintln!("🔍 [PARSED CONFIG] cors_origin bytes = {:?}", cfg.cors_origin.as_bytes());

    let pool: sqlx::Pool<sqlx::Postgres> = PgPoolOptions::new()
        .max_connections(10)
        .connect(&cfg.database_url)
        .await
        .expect("failed to connect to database");

    // Запуск миграций - асинхронный
    migrate::run(&pool).await.expect("migrations failed");

    let addr = format!("{}:{}", cfg.host, cfg.port);
    println!("→ listening on http://{}", addr);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&cfg.cors_origin)
            .allowed_methods(vec!["GET","POST","OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::AUTHORIZATION,
            ])
            .supports_credentials()
            .max_age(600);

        App::new()
            .wrap(Logger::default())
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(cfg.clone()))
            .configure(presentation::routes::configure)
    })
    .bind(addr)?
    .run()
    .await
}
use std::env;
use actix_web::{web, App, HttpResponse, HttpServer};
use actix_web::http::KeepAlive;
use actix_web::middleware::Logger;
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use crate::http::routes::api::configure_routes;
use crate::services::auth_service::AuthService;
use crate::services::token_service::TokenService;
use crate::services::user_service::UserService;
use tracing::{info, error};
use tracing_subscriber::{
    fmt::{format::FmtSpan, time::UtcTime},
    layer::SubscriberExt,
    util::SubscriberInitExt,
    EnvFilter,
};
mod entities;
mod services;
mod middlewares;
mod http;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let formatter = tracing_subscriber::fmt::layer()
        .with_timer(UtcTime::rfc_3339())
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true)
        .with_line_number(true)
        .with_span_events(FmtSpan::FULL)
        .pretty();

    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| {
            EnvFilter::new("info,auth-rs=debug,actix_web=info,sqlx=warn")
        });

    tracing_subscriber::registry()
        .with(env_filter)
        .with(formatter)
        .init();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL не установлен в .env файле (╥﹏╥)");
    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse::<u16>()
        .expect("PORT должен быть числом");

    info!("Подключаемся к базе данных... (◕‿◕✿)");
    let pool = PgPoolOptions::new()
        .max_connections(num_cpus::get() as u32 * 2) // Оптимальное количество
        .min_connections(num_cpus::get() as u32) // Держим минимальный пул готовым
        .acquire_timeout(std::time::Duration::from_secs(3))
        .idle_timeout(std::time::Duration::from_secs(10))
        .connect(&database_url)
        .await
        .expect("Не могу подключиться к базе данных (╥﹏╥)");
    info!("Подключение к базе данных установлено! ٩(◕‿◕｡)۶");

    info!("Применяем миграции... (｡♥‿♥｡)");
    match sqlx::migrate!("src/migrations").run(&pool).await {
        Ok(_) => info!("Миграции успешно применены! (ﾉ◕ヮ◕)ﾉ*:･ﾟ✧"),
        Err(e) => {
            error!("Ошибка при применении миграций: {}", e);
            panic!("Не удалось применить миграции ｡･ﾟﾟ*(>д<)*ﾟﾟ･｡");
        }
    }

    let token_service = TokenService::new(pool.clone());
    let auth_service = AuthService::new(pool.clone(), token_service.clone());
    let user_service = UserService::new(pool.clone(), token_service.clone());

    info!("Сервер запускается на http://{}:{}", host, port);

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Compress::default())
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(user_service.clone()))
            .configure(configure_routes)
            .app_data(web::JsonConfig::default()
                .limit(4096)
                .error_handler(|err, _| {
                    // Быстрая обработка ошибок
                    actix_web::error::InternalError::from_response(
                        err, HttpResponse::BadRequest().finish()
                    ).into()
                }))
    })
        .backlog(1024)
        .workers(num_cpus::get())
        .keep_alive(KeepAlive::Os)
        .bind((host, port))?
        .run()
        .await
}

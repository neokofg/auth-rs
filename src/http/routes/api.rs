use actix_web::web;
use crate::http::controllers::auth_controllers::{login, logout, register};
use crate::http::controllers::user_controllers::get_user;

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/v1")
                    .service(
                        web::scope("/auth")
                            .route("/register", web::post().to(register))
                            .route("/login", web::post().to(login))
                            .route("/logout", web::post().to(logout))
                    )
                    .service(
                        web::scope("/user")
                            .route("/", web::get().to(get_user))
                    )
            )
    );
}
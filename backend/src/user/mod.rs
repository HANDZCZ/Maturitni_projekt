use actix_web::web;
use actix_web::{error, error::JsonPayloadError};

mod hash_utils;
mod login;
pub mod model;
mod register;
mod update;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/user")
            .app_data(
                web::JsonConfig::default()
                    .limit(512)
                    .error_handler(|err, _req| match err {
                        JsonPayloadError::ContentType => error::ErrorBadRequest(""),
                        JsonPayloadError::Overflow => error::ErrorPayloadTooLarge(""),
                        JsonPayloadError::Payload(_) => error::ErrorBadRequest("Invalid data"),
                        JsonPayloadError::Deserialize(_) => error::ErrorBadRequest("Invalid json"),
                    }),
            )
            .service(web::resource("/register").route(web::post().to(register::register)))
            .service(web::resource("/login").route(web::post().to(login::login)))
            .service(web::resource("/update").route(web::post().to(update::update_self))),
    )
    .service(web::resource("/admin/user/update").route(web::post().to(update::update_user)));
}

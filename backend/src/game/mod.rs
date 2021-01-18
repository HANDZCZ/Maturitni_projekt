use actix_web::web;
use actix_web::{error, error::JsonPayloadError};

mod actions;
mod model;
mod invite;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/game")
		    .app_data(
                web::JsonConfig::default()
                    .limit(2048)
                    .error_handler(|err, _req| match err {
                        JsonPayloadError::ContentType => error::ErrorBadRequest(""),
                        JsonPayloadError::Overflow => error::ErrorPayloadTooLarge(""),
                        JsonPayloadError::Payload(_) => error::ErrorBadRequest("Invalid data"),
                        JsonPayloadError::Deserialize(_) => error::ErrorBadRequest("Invalid json"),
                    }),
            )
            .service(
                web::scope("/invite")
                    .service(web::resource("/new").route(web::post().to(invite::new)))
                    .service(web::resource("/get").route(web::get().to(invite::get)))
                    .service(web::resource("/update").route(web::post().to(invite::update))),
            )
            .service(web::resource("/play").route(web::post().to(actions::play))),
    )
    .service(web::resource("/admin/game/invite/new").route(web::post().to(invite::admin_new)));
}

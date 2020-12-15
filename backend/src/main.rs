use actix_web::{middleware, App, HttpServer};
use structopt::StructOpt;
use thiserror::Error;
use time::NumericalDuration;

mod game;
mod macros;
mod session;
mod user;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, env)]
    server_addres: String,
    #[structopt(long, env)]
    auth_ttl: u32,
    #[structopt(long, env)]
    auth_key: String,
    #[structopt(long, env)]
    redis_addres: String,
    #[structopt(long, env)]
    database_url: String,
}

#[derive(Error, Debug)]
enum Error {
    #[error("Dotenv error: {0}")]
    Dotenv(#[from] dotenv::Error),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Db error: {0}")]
    Db(#[from] sqlx::Error),
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv()?;
    env_logger::init();

    let Opt {
        server_addres,
        auth_ttl,
        auth_key,
        redis_addres,
        database_url,
    } = Opt::from_args();

    let pool = sqlx::postgres::PgPoolOptions::new()
        .connect(&database_url)
        .await?;

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                actix_redis::RedisSession::new(redis_addres.clone(), auth_key.as_bytes())
                    .cookie_same_site(actix_web::cookie::SameSite::Strict)
                    .cookie_http_only(true)
                    .ttl(auth_ttl)
                    .cookie_max_age(auth_ttl.seconds())
                    .cookie_name("Abraka Dabraka skoc mi na draka"),
            )
            .data(pool.clone())
            .configure(user::config)
            .configure(game::config)
    })
    .bind(server_addres)?
    .run()
    .await?;

    Ok(())
}

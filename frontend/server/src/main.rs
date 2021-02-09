use actix_web::{get, middleware, App, HttpRequest, HttpServer};
use structopt::StructOpt;
use thiserror::Error;

#[derive(StructOpt)]
struct Opt {
    #[structopt(long, env)]
    server_addres: String,
    #[structopt(long, env)]
    api_domain: String,
}

#[derive(Error, Debug)]
enum Error {
    #[error("Dotenv error: {0}")]
    Dotenv(#[from] dotenv::Error),
    #[error("Io error: {0}")]
    Io(#[from] std::io::Error),
}

#[get("/{filename:.*}")]
async fn index(req: HttpRequest) -> actix_files::NamedFile {
    let path: &str = req.match_info().query("filename");
    let file = actix_files::NamedFile::open("static/".to_owned() + path);
    let out = if path == "" || file.is_err() {
        actix_files::NamedFile::open("static/index_edited.html").unwrap()
    } else {
        file.unwrap()
    };
    out.use_last_modified(true).use_etag(true)
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    dotenv::dotenv()?;
    env_logger::init();

    let Opt {
        server_addres,
        api_domain,
    } = Opt::from_args();

    {
        use std::io::prelude::*;
        let mut file = std::fs::File::open("static/index.html")?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let new_contents = contents.replacen("DO_NOT_EDIT_this_will_get_replaced_by_env_variable", &api_domain, 1);
        let mut new_file = std::fs::File::create("static/index_edited.html")?;
        new_file.write_all(new_contents.as_bytes())?;
    }

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .service(index)
    })
    .bind(server_addres)?
    .run()
    .await?;

    Ok(())
}

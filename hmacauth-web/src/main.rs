use actix_web::{App, HttpRequest, HttpServer};
use log::info;
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::middleware::Logger;

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./statics/index.html")?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting server at http://:8080");

    HttpServer::new(move ||
                        App::new()
                            .wrap(Logger::default())
                            .route("/", actix_web::web::get().to(index))
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

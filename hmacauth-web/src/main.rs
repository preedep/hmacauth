use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use log::{error, info};

use hmacauth_lib::{models, utils};

async fn payload_handler(req: HttpRequest, payload: web::Json<models::Payload>) -> HttpResponse {
    let headers = req.headers();
    let header_maps = headers.iter().map(|(key, value)| {
        (key.to_string(), value.to_str().unwrap().to_string())
    }).collect::<Vec<(String, String)>>();
    info!("Received headers: {:#?}", header_maps);
    let signed_header = utils::get_signed_header(&header_maps);
    info!("Received signed header: {:#?}", signed_header);
    if signed_header.is_err() {
        error!("Unauthorized");
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
    let payload = payload.into_inner();
    info!("Received payload: {:#?}", payload);
    HttpResponse::Ok().json(payload)
}

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting server at http://:8080");

    HttpServer::new(move ||
                        App::new()
                            .wrap(Logger::default())
                            .service(fs::Files::new("/static", "./static"))
                            .route("/", actix_web::web::get().to(index))
                            .route("/apis/v1/payload", web::post().to(payload_handler)

                            )
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

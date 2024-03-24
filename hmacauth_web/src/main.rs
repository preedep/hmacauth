use std::env;
use actix_files as fs;
use actix_files::NamedFile;
use actix_web::{App, HttpRequest, HttpResponse, HttpServer, web};
use actix_web::middleware::Logger;
use log::{debug, error, info};

use hmacauth_lib::{models, utils};

fn get_full_url(req: &HttpRequest) -> String {
    // Get the base URL (request's scheme, host, and port)
    let base_url = format!("{}://{}", req.connection_info().scheme(), req.connection_info().host());

    // Get the request's path and query string
    let path = req.path();
    let query_string = req.query_string();

    // Combine the base URL, path, and query string
    let full_url = format!("{}{}{}", base_url, path, query_string);

    full_url
}

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

    let payload_str = serde_json::to_string(&payload).unwrap();
    let request_url = url::Url::parse(get_full_url(&req).as_str()).unwrap();
    debug!("request_url : {}", request_url);
    let x_ms_date = headers.get("x-ms-date").unwrap().to_str().unwrap().to_string();

    let params = vec![&x_ms_date];

    let (compute_hash, string_to_sign) = utils::generate_string_to_sign(
        &request_url,
        &req.method().to_string(),
        &params,
        &payload_str,
    );
    debug!("compute_hash : {}", compute_hash);
    debug!("string_to_sign : {}", string_to_sign);

    let signature = utils::compute_signature(&string_to_sign, &"IbNSH3Lc5ffMHo/wnQuiOD4C0mx5FqDmVMQaAMKFgaQ=".to_string());
    debug!("signature : {}", signature);

    if signature.eq(&signed_header.unwrap().signature) {
        let payload = payload.into_inner();
        info!("Received payload: {:#?}", payload);
        HttpResponse::Ok().json(payload)
    } else {
        error!("Unauthorized");
        return HttpResponse::Unauthorized().body("Unauthorized");
    }
}

async fn index(_req: HttpRequest) -> actix_web::Result<NamedFile> {
    Ok(NamedFile::open("./static/index.html")?)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    pretty_env_logger::init();
    info!("Starting server at http://:8080");

    let static_folder = env::var("STATIC_FOLDER").unwrap_or_else(|_| "./static".to_string());

    HttpServer::new(move ||
        App::new()
            .wrap(Logger::default())
            .service(fs::Files::new("/static", &static_folder))
            .route("/", actix_web::web::get().to(index))
            .route("/apis/v1/payload", web::post().to(payload_handler),
            )
    )
        .bind(("0.0.0.0", 8080))?
        .run()
        .await
}

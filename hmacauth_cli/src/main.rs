use std::time::Instant;

use clap::Parser;
use log::{error, info};

use hmacauth_lib::models::Payload;
use hmacauth_lib::utils::get_request_header;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    url: String,
    /// Request ID
    #[arg(short, long)]
    request_id: String,
    /// Message to send
    #[arg(short, long)]
    message: String,
    /// Access key
    #[arg(short, long)]
    access_key: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    let payload = Payload {
        message: Some(args.message.clone()),
    };
    let payload_str = serde_json::to_string(&payload).unwrap();
    let method = "POST".to_string();

    let result = get_request_header(
        &args.url.parse().unwrap(),
        &method,
        &args.request_id,
        &payload_str,
        &args.access_key,
    );

    match result {
        Ok(header) => {
            header.iter().for_each(|(key, value)| {
                info!("{}: {}", key, value);
            });
            //let mut header_map = HeaderMap::new();
            let header_map = (&header).try_into().expect("valid headers");

            let start = Instant::now();
            let result = reqwest::Client::new()
                .post(&args.url)
                .headers(header_map)
                .json(&Payload {
                    message: Some(args.message.clone()),
                })
                .send()
                .await;

            // Calculate elapsed time
            let elapsed = start.elapsed();

            match result {
                Ok(response) => {
                    if response.status().is_success() {
                        info!(
                    "Success: {} with Execution time: {:.2?}",
                    response.status(),
                    elapsed
                );
                    } else {
                        error!(
                    "Error: {} with Execution time: {:.2?}",
                    response.status(),
                    elapsed
                );
                    }
                    info!("Response: {:#?}", response);
                }
                Err(e) => {
                    panic!("{}", e);
                }
            }
        }
        Err(e) => {
            //error!("{}", e);
            panic!("{}", e);
        }
    }
    //println!("Hello, world!");
}

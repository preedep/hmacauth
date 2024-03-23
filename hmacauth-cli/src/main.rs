use clap::Parser;
use log::{error, info};
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
    let result = get_request_header(
        &args.url.parse().unwrap(),
        "POST",
        &args.request_id,
        &args.message,
        &args.access_key,
    );
    match result {
        Ok(header) => {
            header.iter().for_each(|(key, value)| {
                info!("{}: {}", key, value);
            });
        }
        Err(e) => {
            error!("{}", e);
        }
    }

    //println!("Hello, world!");
}

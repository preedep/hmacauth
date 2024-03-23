use clap::Parser;

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

    println!("Hello, world!");
}

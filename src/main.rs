mod server;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "[::1]:50051")]
    api_addr: String,
    #[arg(short, long, default_value = "[::1]:50052")]
    raft_addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let opts = server::ServerOptions {
        api_addr: &args.api_addr,
        raft_addr: &args.raft_addr,
    };
    server::start_server(&opts).await?;
    Ok(())
}

mod balancer;
mod config;

use std::sync::Arc;
use anyhow::Result;
use clap::Parser;
use tracing::info;

#[derive(Parser)]
#[command(name = "zebrax", about = "Lightweight HTTP reverse proxy")]
struct Cli {
    #[arg(short, long, default_value = "zebrax.toml")]
    config: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();
    let cfg = config::load(&cli.config)?;
    let lb = Arc::new(balancer::RoundRobin::new(cfg.backends.clone()));

    info!(listen = %cfg.listen, backends = lb.len(), "zebrax starting");
    // Proxy listener wired in subsequent commits.
    Ok(())
}

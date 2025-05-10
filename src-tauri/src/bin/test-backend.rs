use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;

use whitenoise_lib::setup_logging;

/// Test backend for Whitenoise
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    /// Directory for application data
    #[clap(long, value_name = "PATH", required = true)]
    data_dir: PathBuf,

    /// Directory for application logs
    #[clap(long, value_name = "PATH", required = true)]
    logs_dir: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Get directories from args
    let data_dir = args.data_dir;
    let logs_dir = args.logs_dir;

    let env_suffix = if cfg!(dev) { "dev" } else { "release" };
    let formatted_data_dir = data_dir.join(env_suffix);
    let formatted_logs_dir = logs_dir.join(env_suffix);

    std::fs::create_dir_all(&formatted_data_dir)
        .with_context(|| format!("Failed to create data directory: {:?}", formatted_data_dir))?;
    std::fs::create_dir_all(&formatted_logs_dir)
        .with_context(|| format!("Failed to create logs directory: {:?}", formatted_logs_dir))?;

    setup_logging(formatted_logs_dir.clone())
        .map_err(|e| anyhow::anyhow!("Failed to setup logging: {}", e))?;

    tracing::debug!("Logging initialized in directory: {:?}", formatted_logs_dir);
    tracing::debug!("Data directory: {:?}", formatted_data_dir);

    Ok(())
}

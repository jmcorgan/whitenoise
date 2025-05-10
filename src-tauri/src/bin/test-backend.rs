use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use whitenoise_lib::core::{WhitenoiseConfig, WhitenoiseCore};

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

    let config = WhitenoiseConfig::new(&args.data_dir, &args.logs_dir);
    let core = WhitenoiseCore::new(config);

    println!("{:?}", core);

    Ok(())
}

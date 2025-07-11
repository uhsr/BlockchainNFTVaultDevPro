// src/main.rs
/*
 * Main executable for BlockchainNFTVaultDevPro
 */

use clap::Parser;
use blockchainnftvaultdevpro::{Result, run};

#[derive(Parser)]
#[command(version, about = "BlockchainNFTVaultDevPro - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}

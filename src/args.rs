//! Parses command line arguments
//!
//! This module contains the Args struct and its implementation for parsing command line arguments.
//!

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    // Where to read configuration from.
    #[clap(long)]
    pub config: Option<String>,

    // Uninstall instead of install.
    #[clap(long)]
    pub cleanup: bool,
}

impl Args {
    pub fn parse() -> Self {
        let args = Args::try_parse().unwrap_or_else(|e| e.exit());

        let config = args.config;

        let cleanup = args.cleanup;

        Self { config, cleanup }
    }
}

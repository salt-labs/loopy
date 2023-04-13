//! Parses command line arguments
//!
//! This module contains the Args struct and its implementation for parsing command line arguments.
//!

use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// The file path to read configuration data from.
    #[clap(short, long, default_value = "loopy.yaml")]
    pub config: Option<String>,

    /// The action to perform.
    /// Can be either --install or --uninstall.
    #[clap(short, long)]
    pub action: Option<String>,
}

impl Args {
    pub fn parse() -> Self {
        let args = Args::try_parse().unwrap_or_else(|e| e.exit());

        // Validate the provided arguments before continuing.
        validate_args(&args).unwrap_or_else(|e| {
            eprintln!("{}", e);
            std::process::exit(1);
        });

        let config = args.config;

        let action = args.action;

        Self { config, action }
    }
}

fn validate_args(_args: &Args) -> Result<(), String> {
    // TODO: Validate the provided arguments.
    Ok(())
}

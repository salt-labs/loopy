//! loopy binary
//!
//! ## Overview
//!
//! loopy is a command line utility for assisting with
//! a kubernetes packaging inner feedback loop.
//!
//! loopy is about quickly installing or uninstalling packaging like
//! helm charts or carvel kapp packages into a kind cluster.
//!
//! ## Usage
//!
//! The first thing you need to do is create a configuration file. There is
//! an example configuration file in the examples folder within this repository.
//!
//! You can run loopy with `--config` pointing to the configuration file and by
//! default will look for `loopy.yaml` in the current directory.
//!
//! Additional usage information for the ```loopy``` application is available
//! by running ```loopy --help```.
//!

use crate::fortune::show_fortune;
use crate::helm::{helm_chart, helm_repo};
use crate::kubectl::{kubectl_apply_manifest, kubectl_delete_manifest, kubectl_namespace};
use crate::utils::{
    check_command_in_path, create_dir, download_tool, figlet, run_command, update_path,
};
use anyhow::{Context, Result};
use log::{debug, info, warn, LevelFilter};
use std::env;
use std::io;
use std::path::PathBuf;
use std::str::FromStr;

mod args;
mod config;
mod fortune;
mod helm;
mod kubectl;
mod logger;
mod msvc;
mod utils;

// Constants.
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const VENDOR_PATH: &str = "vendor";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Build dependencies for Windows.
    #[cfg(target_env = "msvc")]
    msvc::link_libraries();

    // Let's start the loop.
    let figlet_msg: String = "start".to_string();
    figlet(figlet_msg.as_str(), None, None, None);
    println!("{} has started.", PACKAGE_NAME);

    // Parse the command line arguments
    let args = args::Args::parse();

    // Destructure Args back into individual vars
    let args::Args { config, action, .. } = args;

    // Load the configuration from the file.
    let config_loaded = match config {
        Some(file) => {
            // Ensure the config file isn't an empty string.
            if file.is_empty() {
                println!("The config file path cannot be empty.");
                std::process::exit(1);
            }
            // Return the loaded config file
            config::load_config(&file)?
        }

        None => {
            println!("No config file was provided.");
            std::process::exit(1);
        }
    };

    // Set up logging
    let log_level = config_loaded
        .log
        .as_ref()
        .and_then(|log| log.level.as_ref())
        .and_then(|level| LevelFilter::from_str(level).ok())
        .unwrap_or(LevelFilter::Info);
    let log_file = config_loaded
        .log
        .as_ref()
        .and_then(|log| log.file.as_ref())
        .cloned();
    let fortune_enabled = config_loaded
        .log
        .as_ref()
        .and_then(|log| log.fortune.as_ref())
        .unwrap_or(&false);
    logger::setup_logging(log_level, log_file)?;

    info!("Logging initialized with level: {:?}", log_level);

    // Define where any downloaded tools will be stored.
    let vendor_dir = PathBuf::from(VENDOR_PATH);

    // Update the PATH environment variable to include the vendor directory.
    // This is so that any tools that were previously downloaded will be found.
    update_path(&vendor_dir);

    // Create a reusable reqwest client.
    let client = reqwest::Client::builder().build()?;

    // Check if all required CLI dependencies are present in the PATH.
    // If not, prompt the user to download them if a URL was provided.
    // If no URL, print a message telling the user to download the tool manually and exit.
    for tool in &config_loaded.dependencies.tools {
        debug!("Checking for {}...", tool.name);

        if check_command_in_path(&tool.bin).is_err() {
            warn!("{} is not found in PATH", tool.name);

            // If a URL was provided, prompt the user to download the tool.
            if tool.url.as_ref().map_or(true, |url| url.is_empty()) {
                println!(
                    "Please download the required tool {} and add it to your PATH",
                    tool.name
                );
                std::process::exit(1);
            } else {
                println!(
                    "{} is not found in PATH. Do you want to download it? [Y/n]",
                    tool.name
                );

                let mut user_input = String::new();
                io::stdin().read_line(&mut user_input)?;

                let user_input = user_input.trim().to_lowercase();
                if user_input == "y" || user_input == "yes" || user_input.is_empty() {
                    println!("Downloading {}...", tool.name);

                    // Make sure the vendor directory exists.
                    create_dir(&vendor_dir)?;

                    // Download the tool and place in the vendor directory.
                    let binary_path =
                        download_tool(&client, tool.url.as_ref().unwrap(), &tool.name, &vendor_dir)
                            .await?;
                    println!("Successfully downloaded {}", tool.name);

                    // Update the PATH environment variable to include the vendor directory.
                    update_path(&vendor_dir);

                    // Make the downloaded tool executable.
                    run_command("chmod", &["+x", (binary_path.to_str().unwrap())])?;

                    println!("{} is now available in PATH", tool.name);
                } else {
                    println!("Please download {} and add it to PATH", tool.name);
                    std::process::exit(1);
                }
            }
        } else {
            info!("{} found in PATH", tool.name);
        }
    }
    info!("All required tools are now present in PATH");

    // Perform a match based on the provided action to perform
    // or exit if no action was provided.
    match action.as_deref() {
        Some("install") => {
            println!("Install mode activated...");

            // Install all Helm repositories (dependencies)
            for repo in &config_loaded.dependencies.helm.repositories {
                println!("Adding Helm repository: {}", repo.name);
                let err_msg = format!("Failed to add Helm repository {}", repo.name);
                helm_repo("install", Some(&repo.name), Some(&repo.url))
                    .await
                    .context(err_msg)?;
                println!("Successfully added Helm repository: {}", repo.name);
            }

            // Install all Helm repositories (applications)
            for repo in &config_loaded.application.helm.repositories {
                println!("Adding Helm repository: {}", repo.name);
                let err_msg = format!("Failed to add Helm repository {}", repo.name);
                helm_repo("install", Some(&repo.name), Some(&repo.url))
                    .await
                    .context(err_msg)?;
                println!("Successfully added Helm repository: {}", repo.name);
            }

            // Update all Helm repositories.
            println!("Updating Helm repositories...");
            let err_msg = "Failed to update Helm repositories".to_string();
            helm_repo("update", None, None).await.context(err_msg)?;
            println!("Successfully updated Helm repositories");

            // Install all Manifests (dependencies)
            if config_loaded.dependencies.manifests.is_empty() {
                println!(
                    "No Kubernetes dependency manifests were found in the configuration file. Skipping..."
                );
            } else {
                for manifest in &config_loaded.dependencies.manifests {
                    println!("Installing Kubernetes manifests for: {}", manifest.name);
                    let err_msg = format!(
                        "Failed to install Kubernetes manifests for {}",
                        manifest.name
                    );
                    kubectl_apply_manifest(manifest).await.context(err_msg)?;
                    println!(
                        "Successfully installed Kubernetes manifest: {}",
                        manifest.name
                    );
                }
            }

            // Install all Helm charts (dependencies)
            if config_loaded.dependencies.helm.charts.is_empty() {
                println!(
                    "No Helm chart dependencies were found in the configuration file. Skipping..."
                );
            } else {
                for chart in &config_loaded.dependencies.helm.charts {
                    println!("Installing Helm chart: {}", chart.name);
                    let err_msg = format!("Failed to install Helm chart {}", chart.name);
                    helm_chart("install", chart).context(err_msg)?;
                    println!("Successfully installed Helm chart: {}", chart.name);
                }
            }

            // Install all Manifests (applications)
            if config_loaded.application.manifests.is_empty() {
                println!(
                    "No Kubernetes application manifests were found in the configuration file. Skipping..."
                );
            } else {
                for manifest in &config_loaded.application.manifests {
                    println!("Installing Kubernetes manifests for: {}", manifest.name);
                    let err_msg = format!(
                        "Failed to install Kubernetes manifests for {}",
                        manifest.name
                    );
                    kubectl_apply_manifest(manifest).await.context(err_msg)?;
                    println!(
                        "Successfully installed Kubernetes manifest: {}",
                        manifest.name
                    );
                }
            }

            // Install all Helm charts (applications)
            if config_loaded.application.helm.charts.is_empty() {
                println!(
                    "No Helm chart applications were found in the configuration file. Skipping..."
                );
            } else {
                for chart in &config_loaded.application.helm.charts {
                    println!("Installing Helm chart: {}", chart.name);
                    let err_msg = format!("Failed to install Helm chart {}", chart.name);
                    helm_chart("install", chart).context(err_msg)?;
                    println!("Successfully installed Helm chart: {}", chart.name);
                }
            }

            // Create the loopy namespace.
            println!("Creating namespace: {}", PACKAGE_NAME);
            let err_msg = format!("Failed to create namespace {}", PACKAGE_NAME);
            kubectl_namespace("create", PACKAGE_NAME)
                .await
                .context(err_msg)?;
        }

        Some("uninstall") => {
            println!("Un-install mode activated...");

            // Uninstall all Helm releases (applications)
            for chart in &config_loaded.application.helm.charts {
                println!("Uninstalling Helm chart: {}", chart.name);
                let err_msg = format!("Failed to uninstall Helm chart {}", chart.name);
                helm_chart("uninstall", chart).context(err_msg)?;
                println!("Successfully uninstalled Helm chart: {}", chart.name);
            }

            // Remove all Kubernetes manifests (applications)
            if config_loaded.application.manifests.is_empty() {
                println!(
                    "No Kubernetes application manifests were defined in the config, skipping."
                );
            } else {
                for manifest in &config_loaded.application.manifests {
                    println!("Removing Kubernetes manifests for: {}", manifest.name);
                    let err_msg = format!(
                        "Failed to remove Kubernetes manifests for {}",
                        manifest.name
                    );
                    kubectl_delete_manifest(manifest).await.context(err_msg)?;
                }
            }

            // Uninstall all Helm releases (dependencies)
            for chart in &config_loaded.dependencies.helm.charts {
                println!("Uninstalling Helm chart: {}", chart.name);
                let err_msg = format!("Failed to uninstall Helm chart {}", chart.name);
                helm_chart("uninstall", chart).context(err_msg)?;
                println!("Successfully uninstalled Helm chart: {}", chart.name);
            }

            // Remove all Kubernetes manifests (dependencies)
            if config_loaded.dependencies.manifests.is_empty() {
                println!(
                    "No Kubernetes dependency manifests were defined in the config, skipping."
                );
            } else {
                for manifest in &config_loaded.dependencies.manifests {
                    println!("Removing Kubernetes manifests for: {}", manifest.name);
                    let err_msg = format!(
                        "Failed to remove Kubernetes manifests for {}",
                        manifest.name
                    );
                    kubectl_delete_manifest(manifest).await.context(err_msg)?;
                }
            }

            // Remove all Helm repositories (applications)
            for repo in &config_loaded.application.helm.repositories {
                println!("Removing Helm repository: {}", repo.name);
                let err_msg = format!("Failed to remove Helm repository {}", repo.name);
                helm_repo("uninstall", Some(&repo.name), None)
                    .await
                    .context(err_msg)?;
                println!("Successfully removed Helm repository: {}", repo.name);
            }

            // Remove all Helm repositories (dependencies)
            for repo in &config_loaded.dependencies.helm.repositories {
                println!("Removing Helm repository: {}", repo.name);
                let err_msg = format!("Failed to remove Helm repository {}", repo.name);
                helm_repo("uninstall", Some(&repo.name), None)
                    .await
                    .context(err_msg)?;
                println!("Successfully removed Helm repository: {}", repo.name);
            }

            // Delete the loopy namespace.
            println!("Deleting namespace: {}", PACKAGE_NAME);
            let err_msg = format!("Failed to delete namespace {}", PACKAGE_NAME);
            kubectl_namespace("delete", PACKAGE_NAME)
                .await
                .context(err_msg)?;
        }

        None => {
            println!("No action was specified, nothing to do. See '--help' for usage information.");
            std::process::exit(0);
        }

        _ => {
            println!("Invalid action. Please provide a valid action (install or uninstall).");
            std::process::exit(1);
        }
    }

    println!("The {} action completed successfully.", action.unwrap());

    // Let's end the loop.
    let figlet_msg: String = "end".to_string();
    figlet(figlet_msg.as_str(), None, None, None);
    println!("{} has finished.", PACKAGE_NAME);

    // If fortune is enabled, tell the user their fortune.
    if &fortune_enabled.to_string() == "true" {
        println!(" ");
        if let Some(fortune_message) = show_fortune() {
            println!("Here is your fortune for today: {}", fortune_message);
        } else {
            eprintln!("Failed to get fortune message");
        }
        println!(" ");
    }

    Ok(())
}

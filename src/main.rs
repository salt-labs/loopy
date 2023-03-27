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
//! default will look for `config.yaml` in the current directory.
//!
//! Additional usage information for the ```loopy``` application is available
//! by running ```loopy --help```.
//!

//use crate::helm::{
//    helm_install_chart, helm_install_repo, helm_uninstall_chart, helm_uninstall_repo,
//};
//use crate::kubectl::{kubectl_apply_dir, kubectl_apply_file, kubectl_delete_dir};
use crate::utils::{
    check_command_in_path, create_dir, download_tool, figlet, run_command, update_path,
};
use anyhow::Result;
use log::{debug, info, warn, LevelFilter};
use std::env;
use std::io;
use std::path::PathBuf;

use std::str::FromStr;

mod args;
mod config;
//mod helm;
//mod kubectl;
mod logger;
mod utils;

const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const VENDOR_PATH: &str = "vendor";
const DEFAULT_CONFIG_FILE: &str = "loopy.yaml";

#[tokio::main]
async fn main() -> Result<()> {
    // Let's begin.
    figlet(PACKAGE_NAME, None, None, None);

    // Parse the command line arguments
    let args = args::Args::parse();

    // Destructure Args back into individual vars
    let args::Args {
        config, cleanup, ..
    } = args;

    // Set config file path, use the provided config file or default to "config.yaml"
    let config_file = match config {
        Some(file) => file,
        None => DEFAULT_CONFIG_FILE.to_string(),
    };

    // Load the configuration from the file.
    //println!("Using config file: {}", config_file);
    let config = config::load_config(&config_file)?;

    // Set up logging
    let log_level = config
        .log
        .as_ref()
        .and_then(|log| log.level.as_ref())
        .and_then(|level| LevelFilter::from_str(level).ok())
        .unwrap_or(LevelFilter::Info);
    let log_file = config
        .log
        .as_ref()
        .and_then(|log| log.file.as_ref())
        .cloned();
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
    for tool in &config.dependencies.tools {
        debug!("Checking for {}...", tool.name);

        if check_command_in_path(&tool.bin).is_err() {
            warn!("{} is not found in PATH", tool.name);

            // If a URL was provided, prompt the user to download the tool.
            if tool.url.as_ref().map_or(true, |url| url.is_empty()) {
                println!("Please download {} and add it to PATH", tool.name);
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

    if cleanup {
        println!("Clean-up mode activated...");

        /*
        // Uninstall all Helm releases (applications)
        for chart in &config.application.helm.charts {
            println!("Uninstalling Helm chart: {}", chart.name);
            helm_uninstall_chart(chart.name)
                .with_context(|| format!("Failed to uninstall Helm chart {}", chart.name))?;
        }

        // Uninstall all Helm releases (dependencies)
        for chart in &config.dependencies.helm.charts {
            println!("Uninstalling Helm chart: {}", chart.name);
            helm_uninstall_chart(chart.name)
                .with_context(|| format!("Failed to uninstall Helm chart {}", chart.name))?;
        }

        // Remove all Helm repositories (applications)
        for repo in &config.application.helm.repositories {
            println!("Removing Helm repository: {}", repo.name);
            helm_uninstall_repo(repo.name)
                .with_context(|| format!("Failed to remove Helm repository {}", repo.name))?;
        }

        // Remove all Helm repositories (dependencies)
        for repo in &config.dependencies.helm.repositories {
            println!("Removing Helm repository: {}", repo.name);
            helm_uninstall_repo(repo.name)
                .with_context(|| format!("Failed to remove Helm repository {}", repo.name))?;
        }

        // Remove all Kubernetes manifests (applications)
        for repo in &config.application.helm.repositories {
            println!("Removing Kubernetes manifests: {}", repo.name);
            kubectl_delete_dir(repo.name)
                .with_context(|| format!("Failed to remove Kubernetes manifests {}", repo.name))?;
        }

        // Remove all Kubernetes manifests (dependencies)
        for repo in &config.dependencies.helm.repositories {
            println!("Removing Kubernetes manifests: {}", repo.name);
            kubectl_delete_dir(repo.name)
                .with_context(|| format!("Failed to remove Kubernetes manifests {}", repo.name))?;
        }
        */
    } else {
        println!("Installer mode activated...");

        /*
        // Install all Helm repositories (dependencies)
        for repo in &config.dependencies.helm.repositories {
            println!("Adding Helm repository: {}", repo.name);
            helm_install_repo(repo.name, repo.url)
                .with_context(|| format!("Failed to add Helm repository {}", repo.name))?;
        }

        // Install all Helm repositories (applications)
        for repo in &config.application.helm.repositories {
            println!("Adding Helm repository: {}", repo.name);
            helm_install_repo(repo.name, repo.url)
                .with_context(|| format!("Failed to add Helm repository {}", repo.name))?;
        }

        // Update all Helm repositories.
        println!("Updating Helm repositories...");
        helm_update_repos().with_context(|| format!("Failed to update Helm repositories"))?;

        // Install Kubernetes Namespace and RBAC manifests (dependencies)
        for chart in &config.dependencies.helm.charts {
            println!("Installing Kubernetes Namespace: {}", chart.name);
            kubectl_apply_file(chart.name, "namespace.yaml").with_context(|| {
                format!(
                    "Failed to install Kubernetes Namespace manifest {}",
                    chart.name
                )
            })?;
            println!("Installing Kubernetes RBAC: {}", chart.name);
            kubectl_apply_file(chart.name, "rbac.yaml").with_context(|| {
                format!("Failed to install Kubernetes RBAC manifest {}", chart.name)
            })?;
        }

        // Install Kubernetes Namespace and RBAC manifests (applications)
        for chart in &config.application.helm.charts {
            println!("Installing Kubernetes Namespace: {}", chart.name);
            kubectl_apply_file(chart.name, "namespace.yaml").with_context(|| {
                format!(
                    "Failed to install Kubernetes Namespace manifest {}",
                    chart.name
                )
            })?;
            println!("Installing Kubernetes RBAC: {}", chart.name);
            kubectl_apply_file(chart.name, "rbac.yaml").with_context(|| {
                format!("Failed to install Kubernetes RBAC manifest {}", chart.name)
            })?;
        }

        // Install all Helm charts as releases (dependencies)
        for chart in &config.dependencies.helm.charts {
            println!("Installing Helm chart: {}", chart.name);
            helm_install_chart(chart.name, chart.repo)
                .with_context(|| format!("Failed to install Helm chart {}", chart.name))?;
        }

        // Install all other Kubernetes manifests (dependencies)
        for chart in &config.dependencies.helm.charts {
            println!("Installing Kubernetes manifests: {}", chart.name);
            kubectl_apply_dir(chart.name).with_context(|| {
                format!("Failed to install Kubernetes manifests {}", chart.name)
            })?;
        }

        // Install all Helm charts as releases (applications)
        for chart in &config.application.helm.charts {
            println!("Installing Helm chart: {}", chart.name);
            helm_install_chart(chart.name, chart.repo)
                .with_context(|| format!("Failed to install Helm chart {}", chart.name))?;
        }

        // Install all other Kubernetes manifests (applications)
        for chart in &config.application.helm.charts {
            println!("Installing Kubernetes manifests: {}", chart.name);
            kubectl_apply_dir(chart.name).with_context(|| {
                format!("Failed to install Kubernetes manifests {}", chart.name)
            })?;
        }
        */
    };

    Ok(())
}

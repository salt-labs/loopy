//! Helm
//!
//! This module contains functions for installing and uninstalling Helm charts and repositories.
//!

use crate::utils::run_command;

use anyhow::{Context, Result};
use log::{debug, error, info};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process::Command;

/// Install the given Helm repository if it doesn't already exist
///
/// # Arguments
///
/// * `name` - The name of the Helm repository
/// * `url` - The URL of the Helm repository
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_install_repo;
///
/// fn main() {
///     let result = helm_install_repo("example", "https://charts.example.com");
///     assert!(result.is_ok());
/// }
/// ```
///
pub fn helm_install_repo(name: &str, url: &str) -> Result<()> {
    info!("Installing Helm repo: {}", name);

    // Check if the helm repo already exists
    let repo_list = run_command("helm", &["repo", "list"])?;
    if !repo_list.contains(name) {
        // If the helm repo doesn't currently exist, add it.
        run_command("helm", &["repo", "add", name, url])
            .with_context(|| format!("Failed to add Helm repository '{}'", name))?;
    }

    Ok(())
}

/// Update all Helm repositories
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_update_repos;
/// fn main() {
///     let result = helm_update_repos();
///     assert!(result.is_ok());
/// }
///```
///
pub fn helm_update_repos() -> Result<()> {
    info!("Updating Helm repos");

    // Run the helm repo update command
    run_command("helm", &["repo", "update"])
        .with_context(|| "Failed to update Helm repositories")?;

    Ok(())
}

//! Uninstall the given Helm chart release
//!
//! # Arguments
//!
//! * `name` - The name of the Helm release to uninstall
//!
//! # Examples
//!
//! ```rust
//! use loopy::helm::helm_uninstall_chart;
//! fn main() {
//!     let result = helm_uninstall_chart("example");
//!     assert!(result.is_ok());
//! }
//! ```
//!
pub fn helm_uninstall_repo(name: &str) -> Result<()> {
    info!("Uninstalling Helm repo: {}", name);

    // Check if the helm repo exists
    let repo_list = run_command("helm", &["repo", "list"])?;
    if repo_list.contains(name) {
        // If the helm repo exists, remove it.
        run_command("helm", &["repo", "remove", name])
            .with_context(|| format!("Failed to remove Helm repository '{}'", name))?;
    }

    Ok(())
}

//! Install or upgrade the given Helm chart release
//!
//! # Arguments
//!
//! * `name` - The name of the Helm release to install or upgrade
//! * `repo` - The Helm repository where the chart is located
//!
//! # Examples
//!
//! ```rust
//! use loopy::helm::helm_install_chart;
//! fn main() {
//!     let result = helm_install_chart("example", "example-repo");
//!     assert!(result.is_ok());
//! }
//! ```
//!
pub fn helm_install_chart(name: &str, repo: &str) -> Result<()> {
    info!("Installing Helm chart: {}", name);

    // Check if the chart directory exists
    let chart_dir = format!("helm/charts/{}", name);
    if !Path::new(&chart_dir).exists() {
        return Err(anyhow::anyhow!(
            "Helm chart directory '{}' does not exist",
            chart_dir
        ));
    }

    // Determine the values file to use
    let values_file = if Path::new(&format!("{}/values.yaml", chart_dir)).exists() {
        format!("{}/values.yaml", chart_dir)
    } else {
        format!("{}/defaults.yaml", chart_dir)
    };

    // Check if the helm release already exists
    let release_list = run_command("helm", &["list", "--namespace", name])?;
    if release_list.contains(name) {
        // If the helm release is already installed, upgrade it.
        run_command(
            "helm",
            &[
                "upgrade",
                name,
                "--values",
                &values_file,
                "--namespace",
                name,
                "--create-namespace",
                &format!("{}/{}", repo, name),
            ],
        )
        .with_context(|| format!("Failed to upgrade Helm chart '{}'", name))?;
    } else {
        // If the helm release doesn't currently exist, install it.
        run_command(
            "helm",
            &[
                "install",
                name,
                "--values",
                &values_file,
                "--namespace",
                name,
                "--create-namespace",
                &format!("{}/{}", repo, name),
            ],
        )
        .with_context(|| format!("Failed to install Helm chart '{}'", name))?;
    }

    Ok(())
}

//! Uninstall the given Helm chart release
//!
//! # Arguments
//!
//! * `name` - The name of the Helm release to uninstall
//!
//! # Examples
//!
//! ```rust
//! use loopy::helm::helm_uninstall_chart;
//! fn main() {
//!     let result = helm_uninstall_chart("example");
//!     assert!(result.is_ok());
//! }
//! ```
//!
pub fn helm_uninstall_chart(name: &str) -> Result<()> {
    info!("Uninstalling Helm chart: {}", name);

    // Check if the helm release exists in the specified namespace
    let release_list = run_command("helm", &["list", "--namespace", name])?;
    if release_list.contains(name) {
        // If the helm release exists, uninstall it.
        run_command("helm", &["uninstall", name, "--namespace", name])
            .with_context(|| format!("Failed to uninstall Helm chart '{}'", name))?;
    }

    Ok(())
}

//! Prepare the given Helm chart by creating its directory and defaults.yaml file if they don't already exist
//!
//! # Arguments
//!
//! * `name` - The name of the Helm chart to prepare
//! * `repo` - The Helm repository where the chart is located
//!
//! # Examples
//!
//! ```rust
//! use loopy::helm::helm_prepare_chart;
//! fn main() {
//!     let result = helm_prepare_chart("example", "example-repo");
//!     assert!(result.is_ok());
//! }
//! ```
//!
pub fn helm_prepare_chart(name: &str, repo: &str) -> Result<()> {
    info!("Preparing Helm chart: {}", name);

    // Create the chart directory if it doesn't exist
    let chart_dir = format!("helm/charts/{}", name);
    if !Path::new(&chart_dir).exists() {
        std::fs::create_dir_all(&chart_dir)
            .with_context(|| format!("Failed to create Helm chart directory '{}'", chart_dir))?;
    }

    // Create the defaults.yaml file if it doesn't exist
    let defaults_file = format!("{}/defaults.yaml", chart_dir);
    if !Path::new(&defaults_file).exists() {
        let default_values =
            run_command("helm", &["show", "values", &format!("{}/{}", repo, name)])?;

        let mut file = File::create(&defaults_file).with_context(|| {
            format!("Failed to create defaults.yaml file at '{}'", defaults_file)
        })?;
        file.write_all(default_values.as_bytes()).with_context(|| {
            format!(
                "Failed to write to defaults.yaml file at '{}'",
                defaults_file
            )
        })?;
    }

    Ok(())
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_helm_update_repos() {
        let result = helm_update_repos();
        assert!(result.is_ok());
    }

    #[test]
    fn test_helm_install_chart() {
        let repo_name = "example-repo";
        let repo_url = "https://charts.example.com";
        let chart_name = "example";

        // Prepare the environment by installing the repo and updating it
        let _ = helm_install_repo(repo_name, repo_url);
        let _ = helm_update_repos();

        // Test the helm_install_chart function
        let result = helm_install_chart(chart_name, repo_name);
        assert!(result.is_ok());
    }

    #[test]
    fn test_helm_uninstall_chart() {
        let repo_name = "example-repo";
        let repo_url = "https://charts.example.com";
        let chart_name = "example";

        // Prepare the environment by installing the repo, updating it, and installing the chart
        let _ = helm_install_repo(repo_name, repo_url);
        let _ = helm_update_repos();
        let _ = helm_install_chart(chart_name, repo_name);

        // Test the helm_uninstall_chart function
        let result = helm_uninstall_chart(chart_name);
        assert!(result.is_ok());
    }
}
*/

//! Helm
//!
//! This module contains functions for installing and uninstalling Helm charts and repositories.
//!

use crate::config::{Chart, Repository};
use crate::utils::run_command;

use anyhow::{Context, Result};
use log::{debug, error, info};
use reqwest::StatusCode;
use std::fs::File;
use std::io::Write;
use std::path::Path;

/// Manage helm repositories.
///
/// # Arguments
///
/// * `action` - The action to perform. Can be either 'install', 'uninstall' or 'update.'
/// * `name` - The name of the Helm repository.
/// * `url` - The URL of the Helm repository.
///
pub async fn helm_repo(action: &str, name: Option<&str>, url: Option<&str>) -> Result<()> {
    match action {
        "install" => {
            if let Some(repo_name) = name {
                if let Some(repo_url) = url {
                    helm_install_repo(repo_name, repo_url).await
                } else {
                    Err(anyhow::anyhow!(
                        "URL is required for installing a Helm repo"
                    ))
                }
            } else {
                Err(anyhow::anyhow!(
                    "Name is required for installing a Helm repo"
                ))
            }
        }

        "uninstall" => {
            if let Some(repo_name) = name {
                helm_uninstall_repo(repo_name)
            } else {
                Err(anyhow::anyhow!(
                    "Name is required for uninstalling a Helm repo"
                ))
            }
        }

        "update" => helm_update_repos(),

        _ => Err(anyhow::anyhow!(
            "Invalid action, only 'install', 'uninstall', or 'update' are allowed"
        )),
    }
}

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
/// let result = helm_install_repo("example", "https://charts.example.com");
/// assert!(result.is_ok());
/// ```
///
async fn helm_install_repo(name: &str, url: &str) -> Result<()> {
    info!("Installing Helm repo: {}", name);

    // Check if the helm repo already exists
    let err_msg = "Failed to list Helm repositories".to_string();
    let (stdout, stderr, status) = run_command("helm", &["repo", "list"]).expect(&err_msg);

    if stderr.contains("Error: no repositories to show") && status.code() == Some(1) {
        // If the command failed with 'Error: no repositories to show',
        // then there are no Helm repos but that's OK.
        debug!("No Helm repos are installed, continuing with installation");
    } else if stdout.contains(name) && status.code() == Some(0) {
        // The Helm repo already exists, so we can skip the installation.
        info!("Helm repo '{}' already exists, skipping installation", name);
    } else if status.code() != Some(0) {
        // For any other status code, the command has failed.
        error!("Failed to list Helm repositories");
        info!("stdout: {}", stdout);
        error!("stderr: {}", stderr);
        return Err(anyhow::anyhow!(
            "Failed to list Helm repositories. Please check the log output above for more information."
        ));
    }
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    // Make a connection to the URL.
    let err_msg = format!("Failed to get URL: {}", url);
    let response = reqwest::get(url).await.context(err_msg)?;

    // Helm repo URLs often give a 403 Forbidden response or 404 Not Found response.
    // We don't want to fail if we get one of these responses.
    let allowed_statuses = [StatusCode::OK, StatusCode::FORBIDDEN, StatusCode::NOT_FOUND];

    // Check that a webserver is listening on the URL, we don't what the response is.
    // This is because most Helm repo URLs will give a
    // 403 Forbidden response or 404 Not Found response.
    if !allowed_statuses.contains(&response.status()) {
        error!("Failed to get URL: {}", url);
        error!("Status code: {}", response.status());
        error!("Response body: {}", response.text().await?);
        return Err(anyhow::anyhow!("Failed to get URL: {}", url));
    } else {
        debug!("URL '{}' is valid", url);
    }

    // Add the Helm repo
    let err_msg = format!("Failed to add Helm repository '{}'", name);
    let (stdout, stderr, status) =
        run_command("helm", &["repo", "add", name, url]).context(err_msg)?;

    // Check the status code of the command.
    if status.code() == Some(0) {
        info!("Helm repo '{}' installed successfully", name);
    } else {
        error!("Failed to add Helm repository '{}'", name);
        info!("stdout: {}", stdout);
        error!("stderr: {}", stderr);
        return Err(anyhow::anyhow!(
            "Failed to add Helm repository '{}'. Please check the log output above for more information.",
            name
        ));
    }
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    Ok(())
}

/// Uninstall the given Helm chart release
///
/// # Arguments
///
/// * `name` - The name of the Helm release to uninstall
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_uninstall_chart;
/// let result = helm_uninstall_chart("example");
/// assert!(result.is_ok());
/// ```
///
fn helm_uninstall_repo(name: &str) -> Result<()> {
    info!("Uninstalling Helm repo: {}", name);

    let err_msg = "Failed to list Helm repositories".to_string();
    let (_stdout, stderr, status) = run_command("helm", &["repo", "list"]).expect(&err_msg);

    if stderr.contains("Error: no repositories to show") && status.code() == Some(1) {
        // If the command failed with 'Error: no repositories to show',
        // then there are no Helm repos but thats OK.
        debug!("No Helm repos are installed, skipping");
        return Ok(());
    } else if status.code() == Some(0) {
        // If the command succeeded, then there are Helm repos installed.
        debug!("There are Helm repos installed, continuing");
    } else {
        // For any other status code, the command has failed.
        return Err(anyhow::anyhow!(
            "Failed to list Helm repositories. Please check the output above for more information."
        ));
    }

    // Check if the helm repo exists.
    let err_msg = "Failed to list Helm repositories".to_string();
    let (stdout, _stderr, status) = run_command("helm", &["repo", "list"]).expect(&err_msg);
    if stdout.contains(name) && status.code() == Some(0) {
        // If the helm repo exists, remove it.
        debug!("Removing Helm repo: {}", name);
        let err_msg = format!("Failed to remove Helm repository '{}'", name);
        run_command("helm", &["repo", "remove", name]).context(err_msg)?;
    } else {
        debug!("Helm repo {} is not installed, skipping", name);
    }

    Ok(())
}

/// Update all Helm repositories
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_update_repos;
/// let result = helm_update_repos();
/// assert!(result.is_ok());
///```
///
fn helm_update_repos() -> Result<()> {
    info!("Updating Helm repos");

    // Run the helm repo update command
    let err_msg = "Failed to update Helm repositories".to_string();
    let (stdout, stderr, status) = run_command("helm", &["repo", "update"]).context(err_msg)?;

    if stderr.contains("Error: no repositories found. You must add one before updating")
        && status.code() == Some(1)
    {
        // If the command failed with 'Error: no repositories found. You must add one before updating',
        // then there are no Helm repos but that's OK.
        debug!("No Helm repos are installed, skipping update");
    } else if status.code() != Some(0) {
        error!("Failed to update Helm repositories");
        info!("stdout: {}", stdout);
        error!("stderr: {}", stderr);
        return Err(anyhow::anyhow!(
            "Failed to update Helm repositories. Please check the log output above for more information."
        ));
    } else {
        info!("Helm repos updated successfully");
    }
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    Ok(())
}

/// Manage Helm chart releases
///
/// # Arguments
///
/// * `action` - The action to perform, either 'install' or 'uninstall'
/// * `chart` - A Helm Chart struct with the name, repo and optional values filename.
///
pub fn helm_chart(action: &str, chart: &Chart) -> Result<()> {
    match action {
        "install" => {
            // Use chart.namespace if set, otherwise use chart.name as the default.
            let namespace = chart.namespace.as_ref().unwrap_or(&chart.name);

            // Call helm_install_chart with the provided values or None if not set.
            helm_install_chart(&chart.name, &chart.repo, namespace, chart.values.as_deref())
        }

        "uninstall" => helm_uninstall_chart(&chart.name),

        "prepare" => helm_prepare_chart(&chart.name, &chart.repo),

        _ => Err(anyhow::anyhow!(
            "Invalid action, only 'install', 'uninstall', or 'prepare' are allowed"
        )),
    }
}

/// Install or upgrade the given Helm chart release
///
/// # Arguments
///
/// * `name` - The name of the Helm release to install or upgrade
/// * `repo` - The Helm repository where the chart is located
/// * `values` - An optional name of the values file to use.
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_install_chart;
/// let result = helm_install_chart("example", "example-repo");
/// assert!(result.is_ok());
/// ```
///
fn helm_install_chart(
    name: &str,
    repo: &str,
    namespace: &str,
    values_filename: Option<&str>,
) -> Result<()> {
    info!("Installing Helm chart: {} into {}", name, namespace);

    // Check if the chart directory exists
    let chart_dir: String = format!("config/helm/{}", name);
    if !Path::new(&chart_dir).exists() {
        info!("Helm chart directory does not exist, preparing chart");
        helm_prepare_chart(name, repo)?;
    } else {
        debug!("Helm chart directory exists: {}", chart_dir);
    }

    // Determine the values file to use.
    let values_file = if let Some(filename) = values_filename {
        let path = format!("{}/{}", chart_dir, filename);

        if !Path::new(&path).exists() {
            return Err(anyhow::anyhow!(
                "Provided values file '{}' does not exist.",
                path
            ));
        } else {
            debug!("Using values file: {}", path);
        }
        path
    } else {
        let default_path = format!("{}/values.yaml", chart_dir);
        debug!("No values file provided, using default: {}", default_path);

        if !Path::new(&default_path).exists() {
            // If the default values file doesn't exist, create it.
            debug!(
                "A default values file does not exist, creating one now: {}",
                default_path
            );
            helm_prepare_chart(name, repo)?;
        }

        default_path
    };

    // Check if the helm release already exists
    let (stdout, stderr, status) = run_command("helm", &["list", "--namespace", namespace])?;
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    // Handle the error condition first.
    if status.code() != Some(0) {
        let error_msg = format!("Failed to list Helm releases: {}", stderr);
        return Err(anyhow::anyhow!(error_msg));
    } else if stdout.contains(name) {
        debug!("Helm release already exists, upgrading: {}", name);
        // If the helm release is already installed, upgrade it.
        let (stdout, stderr, status) = run_command(
            "helm",
            &[
                "upgrade",
                name,
                "--values",
                &values_file,
                "--namespace",
                namespace,
                "--wait",
                "--timeout",
                "10m0s",
                &format!("{}/{}", repo, name),
            ],
        )
        .with_context(|| format!("Failed to upgrade Helm chart '{}'", name))?;
        debug!("stdout: {}", stdout);
        debug!("stderr: {}", stderr);

        if status.code() != Some(0) {
            let error_msg = format!("Failed to upgrade Helm chart: {}", stderr);
            return Err(anyhow::anyhow!(error_msg));
        }
    } else {
        debug!("Helm release does not exist, installing: {}", name);
        // If the helm release doesn't currently exist, install it.
        let (stdout, stderr, status) = run_command(
            "helm",
            &[
                "install",
                name,
                "--values",
                &values_file,
                "--namespace",
                namespace,
                "--create-namespace",
                "--wait",
                "--timeout",
                "10m0s",
                &format!("{}/{}", repo, name),
            ],
        )
        .with_context(|| format!("Failed to install Helm chart '{}'", name))?;
        debug!("stdout: {}", stdout);
        debug!("stderr: {}", stderr);

        if status.code() != Some(0) {
            let error_msg = format!("Failed to install Helm chart: {}", stderr);
            return Err(anyhow::anyhow!(error_msg));
        }
    }

    Ok(())
}

/// Uninstall the given Helm chart release
///
/// # Arguments
///
/// * `name` - The name of the Helm release to uninstall
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_uninstall_chart;
/// let result = helm_uninstall_chart("example");
/// assert!(result.is_ok());
/// ```
///
fn helm_uninstall_chart(name: &str) -> Result<()> {
    info!("Uninstalling Helm chart: {}", name);

    // Check if the helm release exists in the specified namespace
    let (stdout, stderr, status) = run_command("helm", &["list", "--namespace", name])?;
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    // Handle the error condition first.
    if status.code() != Some(0) {
        let error_msg = format!("Failed to list Helm releases: {}", stderr);
        return Err(anyhow::anyhow!(error_msg));
    } else if stdout.contains(name) {
        debug!("Helm chart '{}' is installed, uninstalling.", name);

        // If the helm release exists, uninstall it.
        let (stdout, stderr, status) =
            run_command("helm", &["uninstall", name, "--namespace", name])
                .with_context(|| format!("Failed to uninstall Helm chart '{}'", name))?;
        debug!("stdout: {}", stdout);
        debug!("stderr: {}", stderr);

        if status.code() != Some(0) {
            let error_msg = format!("Failed to uninstall Helm chart: {}", stderr);
            return Err(anyhow::anyhow!(error_msg));
        }
    } else {
        info!("Helm chart '{}' is not installed, skipping.", name)
    }

    Ok(())
}

/// Prepare the given Helm chart by creating its directory and values.yaml file if they don't already exist
///
/// # Arguments
///
/// * `name` - The name of the Helm chart to prepare
/// * `repo` - The Helm repository where the chart is located
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_prepare_chart;
/// let result = helm_prepare_chart("example", "example-repo");
/// assert!(result.is_ok());
/// ```
///
fn helm_prepare_chart(name: &str, repo: &str) -> Result<()> {
    info!("Preparing Helm chart: {}", name);

    // Create the chart directory if it doesn't exist
    let chart_dir: String = format!("config/helm/{}", name);
    if !Path::new(&chart_dir).exists() {
        info!("Creating Helm chart directory: {}", chart_dir);
        let err_msg = format!("Failed to create Helm chart directory '{}'", chart_dir);
        std::fs::create_dir_all(&chart_dir).context(err_msg)?;
    }

    // Create the values.yaml file if it doesn't exist
    let defaults_file = format!("{}/values.yaml", chart_dir);
    if !Path::new(&defaults_file).exists() {
        info!("Creating Helm chart defaults file: {}", defaults_file);
        let (stdout, stderr, status) =
            run_command("helm", &["show", "values", &format!("{}/{}", repo, name)])?;

        // Handle the error condition first.
        if status.code() != Some(0) {
            let error_msg = format!("Failed to get Helm chart values: {}", stderr);
            return Err(anyhow::anyhow!(error_msg));
        }

        let err_msg = format!(
            "Failed to create Helm chart defaults file '{}'",
            defaults_file
        );
        let mut file = File::create(&defaults_file).context(err_msg)?;
        let err_msg = format!(
            "Failed to write to Helm chart defaults file '{}'",
            defaults_file
        );
        file.write_all(stdout.as_bytes()).context(err_msg)?;
    }

    Ok(())
}

/// Process Helm Repositories
///
/// # Arguments
///
/// * `repos` - The list of Helm repositories to process
/// * `action` - The action to perform on the Helm repositories
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_process_repositories;
/// let result = helm_process_repositories(&[], "update");
/// assert!(result.is_ok());
/// ```
///
pub async fn helm_process_repos(repos: &[Repository], action: &str) -> Result<()> {
    for repo in repos {
        let err_message = format!("Failed to {} Helm repository: {}", action, repo.name);
        println!("{} Helm repository: {}", action, repo.name);
        helm_repo(action, Some(&repo.name), Some(&repo.url))
            .await
            .context(err_message)?;
        println!("Successfully {} Helm repository: {}", action, repo.name);
    }
    Ok(())
}

/// Process Helm Charts
///
/// # Arguments
///
/// * `charts` - The list of Helm charts to process
/// * `action` - The action to perform on the Helm charts
///
/// # Examples
///
/// ```rust
/// use loopy::helm::helm_process_charts;
/// let result = helm_process_charts(&[], "install");
/// assert!(result.is_ok());
/// ```
///
pub async fn helm_process_charts(charts: &[Chart], action: &str) -> Result<()> {
    if charts.is_empty() {
        println!(
            "No Helm chart {} were found in the configuration file. Skipping...",
            action
        );
    } else {
        for chart in charts {
            println!("{} Helm chart: {}", action, chart.name);
            let err_msg = format!("Failed to {} Helm chart {}", action, chart.name);
            helm_chart(action, chart).context(err_msg)?;
            println!("Successfully {} Helm chart: {}", action, chart.name);
        }
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

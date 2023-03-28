//! Kubectl
//!
//! This module contains functions for interacting with Kubernetes using the kubectl CLI.
//!

use crate::{config::Manifests, utils::run_command};

use anyhow::{Context, Result};
use log::{debug, error, info, warn};

/// Kubectl apply URL.
///
/// Applies a single Kubernetes manifest from a URL.
///
/// # Arguments
///
/// * `url` - The URL of the manifest file to apply
///
/// # Examples
///
/// ```rust
/// use loopy::kubectl::kubectl_apply_url;
/// let result = kubectl_apply_url("https://raw.githubusercontent.com/kubernetes/ingress-nginx/master/deploy/static/provider/cloud/deploy.yaml");
/// assert!(result.is_ok());
/// ```
///
/*
pub async fn kubectl_apply_url(_name: &str, url: &str, dry_run: bool) -> Result<()> {
    info!("Applying Kubernetes manifest from URL: {}", url);

    let args = if dry_run {
        vec!["apply", "-f", url, "--dry-run=client"]
    } else {
        vec!["apply", "-f", url]
    };

    // Determine if the URL is valid.
    let err_msg = format!("Failed to get URL: {}", url);
    let response = reqwest::get(url).await.context(err_msg)?;

    // Check the status code of the response.
    if !response.status().is_success() {
        error!("Failed to get URL: {}", url);
        error!("Status code: {}", response.status());
        error!("Response body: {}", response.text().await?);
        return Err(anyhow::anyhow!("Failed to get URL: {}", url));
    }

    // Apply the manifest file using kubectl
    let err_msg = format!("Failed to apply manifest file from URL: {}", url);
    run_command("kubectl", &args).context(err_msg)?;

    Ok(())
}
*/
/// Kubectl delete URL.
///
/// Deletes a single Kubernetes manifest from a URL.
///
/// # Arguments
///
/// * `url` - The URL of the manifest file to delete
///
/// # Examples
///
/// ```rust
/// use loopy::kubectl::kubectl_delete_url;
/// let result = kubectl_delete_url("https://raw.githubusercontent.com/kubernetes/ingress-nginx/master/deploy/static/provider/cloud/deploy.yaml");
/// assert!(result.is_ok());
/// ```
///
pub async fn kubectl_delete_url(url: &str, dry_run: bool) -> Result<()> {
    info!("Deleting Kubernetes manifest from URL: {}", url);

    let args = if dry_run {
        vec!["delete", "-f", url, "--dry-run=client"]
    } else {
        vec!["delete", "-f", url]
    };

    // Determine if the URL is valid.
    let err_msg = format!("Failed to get URL: {}", url);
    let response = reqwest::get(url).await.context(err_msg)?;

    // Check the status code of the response.
    if !response.status().is_success() {
        error!("Failed to get URL: {}", url);
        error!("Status code: {}", response.status());
        error!("Response body: {}", response.text().await?);
        return Err(anyhow::anyhow!("Failed to get URL: {}", url));
    } else {
        debug!("URL is valid: {}", url)
    }

    // Delete the manifest file using kubectl
    debug!("Deleting manifest file from URL: {}", url);
    let err_msg = format!("Failed to delete manifest file from URL: {}", url);
    let (stdout, stderr, status) = run_command("kubectl", &args).context(err_msg)?;

    // Handle the error condition first.
    if status.code() != Some(0) {
        // If the stderr contains "not found" then the manifest file was already deleted and can be ignored.
        if stderr.contains("(NotFound)") {
            warn!(
                "Contents of manifest file {} were not found, likely already deleted. Enable debug logging to see the full error message.", url
            );
        } else {
            error!("Failed to delete manifest file from URL: {}", url);
            error!("stdout: {}", stdout);
            error!("stderr: {}", stderr);
            return Err(anyhow::anyhow!(
                "Failed to delete manifest file from URL: {}",
                url
            ));
        }
    } else {
        info!("Deleted manifest file from URL: {}", url);
    }
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    Ok(())
}

/// Kubectl apply file.
///
/// Applies a single Kubernetes manifest file.
///
/// # Arguments
///
/// * `chart_name` - The name of the chart where the manifest file is located
/// * `filename` - The name of the manifest file to apply
///
/// # Examples
///
/// ```rust
/// use loopy::kubectl::kubectl_apply_file;
/// let result = kubectl_apply_file("example-chart", "namespace.yaml");
/// assert!(result.is_ok());
/// ```
/*
pub fn kubectl_apply_file(name: &str, file: &str, dry_run: bool) -> Result<()> {
    // Construct the manifest file path
    let manifest_path = format!("config/manifests/{}/{}", name, file);
    info!("Applying Kubernetes manifest: {}", manifest_path);

    let args = if dry_run {
        vec!["apply", "-f", &manifest_path, "--dry-run=client"]
    } else {
        vec!["apply", "-f", &manifest_path]
    };

    // Ensure the file exists before applying it.
    if !std::path::Path::new(&manifest_path).exists() {
        error!("Manifest file does not exist: {}", manifest_path);
        return Err(anyhow::anyhow!(
            "Manifest file does not exist: {}",
            manifest_path
        ));
    }

    // Apply the manifest file using kubectl
    let err_msg = format!("Failed to apply manifest file: {}", manifest_path);
    run_command("kubectl", &args).context(err_msg)?;

    Ok(())
}
*/
/// Kubectl apply directory.
///
/// Applies all Kubernetes manifests from a specified directory.
///
/// # Arguments
///
/// * `dir` - The directory where the manifest files are located
/// * `dry_run` - If true, the manifests will be applied in dry-run mode
///
/// # Examples
///
/// ```rust
/// use loopy::kubectl::kubectl_apply_dir;
/// let result = kubectl_apply_dir("example-chart", false);
/// assert!(result.is_ok());
/// ```
///
/*
pub fn kubectl_apply_dir(dir: &str, dry_run: bool) -> Result<()> {
    // Construct the manifest file path
    let manifests_path = format!("config/manifests/{}", dir);
    info!(
        "Applying Kubernetes manifests from directory: {}",
        manifests_path
    );

    let args = if dry_run {
        vec!["apply", "-f", &manifests_path, "--dry-run=client"]
    } else {
        vec!["apply", "-f", &manifests_path]
    };

    // Ensure the directory exists before applying it.
    if !std::path::Path::new(&manifests_path).exists() {
        error!("Manifest directory does not exist: {}", manifests_path);
        return Err(anyhow::anyhow!(
            "Manifest directory does not exist: {}",
            &manifests_path
        ));
    }

    // Apply the manifest files using kubectl
    let err_msg = format!(
        "Failed to apply manifest files from directory: {}",
        &manifests_path
    );
    run_command("kubectl", &args).context(err_msg)?;

    Ok(())
}
*/

/// Kubectl delete directory.
///
/// Deletes all Kubernetes manifests from a specified directory.
///
/// # Arguments
///
/// * `dir` - The directory where the manifest files are located
/// * `dry_run` - If true, the manifests will be applied in dry-run mode
///
/// # Examples
///
/// ```rust
/// use loopy::kubectl::kubectl_delete_dir;
/// let result = kubectl_delete_dir("example-chart");
/// assert!(result.is_ok());
/// ```
///
pub fn kubectl_delete_dir(dir: &str, dry_run: bool) -> Result<()> {
    // Construct the manifest file path
    let manifests_path = format!("config/manifests/{}", dir);
    info!(
        "Deleting Kubernetes manifests from directory: {}",
        manifests_path
    );

    let args = if dry_run {
        vec!["delete", "-f", &manifests_path, "--dry-run=client"]
    } else {
        vec!["delete", "-f", &manifests_path]
    };

    // Ensure the directory exists before deleting it.
    if !std::path::Path::new(&manifests_path).exists() {
        error!("Manifest directory does not exist: {}", manifests_path);
        return Err(anyhow::anyhow!(
            "Manifest directory does not exist: {}",
            &manifests_path
        ));
    }

    // Ensure the path contains at least one YAML file.
    let mut files = std::fs::read_dir(&manifests_path)?;
    if files.next().is_none() {
        error!("Manifest directory is empty: {}", manifests_path);
        return Err(anyhow::anyhow!(
            "Manifest directory is empty: {}",
            &manifests_path
        ));
    }

    // Delete the manifest files using kubectl
    let err_msg = format!(
        "Failed to delete manifest files from directory: {}",
        &manifests_path
    );
    let (stdout, stderr, status) = run_command("kubectl", &args).context(err_msg)?;

    // If the command failed, return an error.
    if status.code() != Some(0) {
        // If the stderr contains "not found" then the manifest file was already deleted and can be ignored.
        if stderr.contains("(NotFound)") {
            warn!(
                "Contents of manifest files from directory: {} were not found, likely already deleted. Enable debug logging to see the full error message.",
                &manifests_path
            );
        } else {
            return Err(anyhow::anyhow!(
                "Failed to delete manifest files from directory: {}",
                &manifests_path
            ));
        }
    } else {
        info!(
            "Successfully deleted Kubernetes manifests from directory: {}",
            manifests_path
        );
    }
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    Ok(())
}

/// Kubectl delete manifests.
///
/// Deletes all Kubernetes manifests from a provided config.
///
/// # Arguments
///
/// * `manifests` - The list of manifests to delete
///
/// # Examples
///
/// ```rust
/// use loopy::kubectl::kubectl_delete_manifests;
/// let result = kubectl_delete_manifests(&manifests);
/// assert!(result.is_ok());
/// ```
///
pub async fn kubectl_delete_manifest(manifest: &Manifests) -> Result<()> {
    match &manifest.url {
        Some(url) => {
            let err_msg = format!(
                "Failed to remove Kubernetes manifests {} URL {}",
                manifest.name, url
            );
            kubectl_delete_url(url, false).await.context(err_msg)?;
            println!(
                "Successfully removed Kubernetes manifests {} URL {}",
                manifest.name, url
            )
        }
        None => {
            debug!(
                "No URL provided for Kubernetes manifests {}, skipping.",
                manifest.name
            )
        }
    }

    // If a directory was provided, delete that next.
    match &manifest.dir {
        Some(dir) => {
            let err_msg = format!(
                "Failed to remove Kubernetes manifests {} using directory {}",
                manifest.name, dir
            );
            kubectl_delete_dir(dir, false).context(err_msg)?;
            println!(
                "Successfully removed Kubernetes manifests {} using directory {}",
                manifest.name, dir
            )
        }
        None => {
            debug!(
                "No directory provided for Kubernetes manifests {}, skipping.",
                manifest.name
            )
        }
    }

    Ok(())
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kubectl_apply_dir() {
        let chart_name = "example-chart";

        // Create a temporary directory for the example chart
        let temp_dir = tempfile::tempdir().unwrap();
        let manifests_dir = temp_dir.path().join("manifests");
        std::fs::create_dir_all(&manifests_dir).unwrap();

        // Create a sample manifest file in the directory
        let sample_manifest = manifests_dir.join("sample.yaml");
        std::fs::write(&sample_manifest, "apiVersion: v1\nkind: ConfigMap").unwrap();

        // Run the kubectl_apply_dir function with --dry-run=client
        let result = kubectl_apply_dir(chart_name, &manifests_dir.to_str().unwrap(), true);
        assert!(result.is_ok());

        // Clean up the temporary directory
        temp_dir.close().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kubectl_delete_dir() {
        let chart_name = "example-chart";

        // Create a temporary directory for the example chart
        let temp_dir = tempfile::tempdir().unwrap();
        let manifests_dir = temp_dir.path().join("manifests");
        std::fs::create_dir_all(&manifests_dir).unwrap();

        // Create a sample manifest file in the directory
        let sample_manifest = manifests_dir.join("sample.yaml");
        std::fs::write(&sample_manifest, "apiVersion: v1\nkind: ConfigMap").unwrap();

        // Run the kubectl_delete_dir function with --dry-run=client
        let result = kubectl_delete_dir(chart_name, &manifests_dir.to_str().unwrap(), true);
        assert!(result.is_ok());

        // Clean up the temporary directory
        temp_dir.close().unwrap();
    }
}

*/

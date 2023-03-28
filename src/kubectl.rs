//! Kubectl
//!
//! This module contains functions for interacting with Kubernetes using the kubectl CLI.
//!

use crate::{config::Manifests, utils::run_command};

use anyhow::{Context, Result};
use log::{debug, error, info, warn};
use std::cmp::Ordering;
use std::path::PathBuf;

/// Kubectl apply or delete a URL.
///
/// Applies or deletes a Kubernetes manifest from a URL.
///
/// # Arguments
///
/// * `action` - The action to perform. Can be either "apply" or "delete"
/// * `url` - The URL to apply or delete
/// * `dry_run` - If true, the manifests will be applied in dry-run mode
///
async fn kubectl_url(action: &str, url: &str, dry_run: bool) -> Result<()> {
    if action != "apply" && action != "delete" {
        return Err(anyhow::anyhow!(
            "Invalid action: {}. Must be 'apply' or 'delete'",
            action
        ));
    }

    info!("{}ing Kubernetes manifest from URL: {}", action, url);

    let args = if dry_run {
        vec![action, "-f", url, "--dry-run=client"]
    } else {
        vec![action, "-f", url]
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

    // Apply or delete the manifest file using kubectl
    debug!("{}ing manifest file from URL: {}", action, url);
    let err_msg = format!("Failed to {} manifest file from URL: {}", action, url);
    let (stdout, stderr, status) = run_command("kubectl", &args).context(err_msg)?;

    // A list of allowed errors that can be safely ignored.
    let allowed_errors = ["(NotFound)", "resource mapping not found"];
    // A closure to check if any allowed errors are present in the stderr message.
    let has_allowed_error = || -> bool { allowed_errors.iter().any(|e| stderr.contains(e)) };

    // Handle the error condition first.
    if status.code() != Some(0) {
        if action == "delete" && has_allowed_error() {
            warn!(
                "Contents of manifest file {} were not found, likely already deleted. Enable debug logging to see the full error message.", url
            );
        } else {
            error!("Failed to {} manifest file from URL: {}", action, url);
            error!("stdout: {}", stdout);
            error!("stderr: {}", stderr);
            return Err(anyhow::anyhow!(
                "Failed to {} manifest file from URL: {}",
                action,
                url
            ));
        }
    } else {
        info!("{}ed manifest file from URL: {}", action, url);
    }
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    Ok(())
}

/// Kubectl apply or delete manifests.
///
/// Applies or deletes Kubernetes manifest files from a directory or a single file based on the action.
///
/// # Arguments
///
/// * `action` - The action to perform, either "apply" or "delete"
/// * `chart_name` - The name of the chart where the manifest files are located
/// * `filename` - Optional: The name of the manifest file to apply or delete. If not provided, all files in the chart directory will be applied or deleted.
///
/// # Examples
///
/// ```rust
/// use loopy::kubectl::kubectl_apply_or_delete_manifests;
/// let result = kubectl_apply_or_delete_manifests("apply", "example-chart", Some("namespace.yaml"));
/// assert!(result.is_ok());
/// ```
///
fn kubectl_manifests(
    action: &str,
    name: &str,
    filename: Option<&str>,
    dry_run: bool,
) -> Result<()> {
    match action {
        "apply" => {}
        "delete" => {}
        _ => {
            return Err(anyhow::anyhow!(
                "Invalid action: {}. Must be 'apply' or 'delete'",
                action
            ))
        }
    };

    let manifest_dir = format!("config/manifests/{}", name);

    if let Some(file) = filename {
        let manifest_path = format!("{}/{}", manifest_dir, file);
        info!("{}ing Kubernetes manifest: {}", action, manifest_path);
        kubectl_manifest_single(action, &manifest_path, dry_run)?;
    } else {
        info!(
            "{}ing all Kubernetes manifests in directory: {}",
            action, manifest_dir
        );

        let entries: Vec<_> = std::fs::read_dir(&manifest_dir)?
            .map(|entry| entry.map(|e| e.path()))
            .collect::<Result<_, _>>()?;

        // Apply or delete the special manifests in their specific order.
        // TODO: Replace this janky manual work with a sorted list using sort_manifest_files
        let file_order = match action {
            "apply" => vec![
                "namespace.yaml",
                "crds.yaml",
                "rbac.yaml",
                "webhook.yaml",
                "install.yaml",
            ],
            "delete" => vec![
                "install.yaml",
                "webhook.yaml",
                "rbac.yaml",
                "crds.yaml",
                "namespace.yaml",
            ],
            _ => {
                return Err(anyhow::anyhow!(
                    "Invalid action: {}. Must be 'apply' or 'delete'",
                    action
                ))
            }
        };

        // Apply the special manifests in their specific order.
        for filename in file_order {
            if let Some(path) = entries.iter().find(|p| p.ends_with(filename)) {
                debug!("{}ing manifest file: {}", action, path.display());
                kubectl_manifest_single(action, &path.to_string_lossy(), dry_run)?;
            }
        }

        // TODO: Fix more jankiness
        // Even more jankiness. Need to sleep in between applying the CRDs
        // and the rest of the manifests. Otherwise, the CRDs are not
        // available when the rest of the manifests are applied.
        let wait_time = 60;
        if action == "apply" {
            println!("Waiting for CRDs to be available...");
            info!("Waiting {} seconds for CRDs to be available...", wait_time);
            std::thread::sleep(std::time::Duration::from_secs(wait_time));
        }

        // Apply or delete the rest of the manifest files in the original order.
        for manifest_path in entries {
            debug!("{}ing manifest file: {}", action, manifest_path.display());
            kubectl_manifest_single(action, &manifest_path.to_string_lossy(), dry_run)?;
        }
    }

    Ok(())
}

/// Sort manifest files based on the action and user-defined priorities.
///
/// A helper function for kubectl_manifests.
///
/// Although you can apply an entire directory directly and have kubectl
/// apply all the files in the directory, we want to apply the files in
/// a specific order. This is because some files depend on other files
/// being applied first. For example, a namespace must be created before a
/// deployment can be created in that namespace. kubectl does not handle this
/// ordering for us, so we have to do it ourselves.
///
/// # Arguments
///
/// * `entries` - The manifest files to be sorted
/// * `action` - The action to perform, either "apply" or "delete"
/// * `priorities` - A list of manifest filenames to prioritize, in the order they should be applied
///
fn _sort_manifest_files(
    entries: &mut [PathBuf],
    action: &str,
    priorities: &[&str],
) -> Vec<PathBuf> {
    // Sort the manifest files based on priority and alphabetical order
    entries.sort_by(|a, b| {
        let a_name = a.file_name().unwrap().to_string_lossy().to_lowercase();
        let b_name = b.file_name().unwrap().to_string_lossy().to_lowercase();

        let a_ext = a
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();
        let b_ext = b
            .extension()
            .unwrap_or_default()
            .to_string_lossy()
            .to_lowercase();

        let order = if action == "apply" {
            Ordering::Less
        } else {
            Ordering::Greater
        };

        let cmp = |x: &str, y: &str| {
            if x == y {
                Ordering::Equal
            } else if priorities.contains(&x) {
                if priorities.contains(&y) {
                    priorities
                        .iter()
                        .position(|&p| p == x)
                        .unwrap()
                        .cmp(&priorities.iter().position(|&p| p == y).unwrap())
                } else {
                    order
                }
            } else if priorities.contains(&y) {
                order.reverse()
            } else {
                x.cmp(y)
            }
        };

        cmp(&a_name, &b_name).then_with(|| a_ext.cmp(&b_ext))
    });

    // Reverse the order for delete actions
    if action == "delete" {
        entries.reverse();
    }

    let sorted_entries: Vec<PathBuf> = entries
        .iter()
        .filter(|entry| entry.is_file())
        .map(|entry| entry.to_path_buf())
        .collect();

    debug!(
        "Sorted manifest files for action '{}': {:?}",
        action,
        sorted_entries
            .iter()
            .map(|e| e.to_string_lossy())
            .collect::<Vec<_>>()
    );

    sorted_entries
}

/// Kubectl apply or delete a single manifest file.
///
/// A helper function for applying or deleting a single manifest file.
/// This function is used by `kubectl_apply_or_delete_manifests`.
/// It is not intended to be used directly.
///
/// # Arguments
///
/// * `action` - The action to perform, either "apply" or "delete"
/// * `manifest_path` - The path to the manifest file to apply or delete
/// * `dry_run` - If true, the manifests will be applied in dry-run mode
///
fn kubectl_manifest_single(action: &str, manifest_path: &str, dry_run: bool) -> Result<()> {
    let args = if dry_run {
        vec![action, "-f", manifest_path, "--dry-run=client"]
    } else {
        vec![action, "-f", manifest_path]
    };

    // Ensure the file or directory exists before applying or deleting it.
    if !std::path::Path::new(manifest_path).exists() {
        error!(
            "Manifest file or directory does not exist: {}",
            manifest_path
        );
        return Err(anyhow::anyhow!(
            "Manifest file or directory does not exist: {}",
            manifest_path
        ));
    }

    // Apply or delete the manifest file using kubectl
    let err_msg = format!("Failed to {} manifest: {}", action, manifest_path);
    let (stdout, stderr, status) = run_command("kubectl", &args).expect(&err_msg);

    // A list of allowed errors that can be safely ignored.
    let allowed_errors = ["(NotFound)", "resource mapping not found"];
    // A closure to check if any allowed errors are present in the stderr message.
    let has_allowed_error = || -> bool { allowed_errors.iter().any(|e| stderr.contains(e)) };

    if status.code() != Some(0) {
        if action == "delete" && has_allowed_error() {
            warn!(
                "Contents of manifest file {} were not found, likely already deleted. Enable debug logging to see the full error message.", manifest_path
            );
        } else {
            error!("Failed to {} manifest: {}", action, manifest_path);
            info!("stdout: {}", stdout);
            error!("stderr: {}", stderr);
            return Err(anyhow::anyhow!(
                "Failed to {} manifest: {}. Please check the log output above for more information.",
                action,
                manifest_path
            ));
        }
    } else {
        info!("Successfully {}ed manifest: {}", action, manifest_path);
    }
    debug!("stdout: {}", stdout);
    debug!("stderr: {}", stderr);

    Ok(())
}

/// Kubectl apply manifests.
///
/// Applies all Kubernetes manifests from a provided config.
///
/// # Arguments
///
/// * `manifests` - The list of manifests to apply
///
/// # Examples
///
/// ```rust
/// use loopy::kubectl::kubectl_apply_manifests;
/// let result = kubectl_apply_manifests(&manifests);
/// assert!(result.is_ok());
/// ```
///
pub async fn kubectl_apply_manifest(manifest: &Manifests) -> Result<()> {
    match &manifest.url {
        Some(url) => {
            let err_msg = format!(
                "Failed to apply Kubernetes manifests {} URL {}",
                manifest.name, url
            );
            kubectl_url("apply", url, false).await.context(err_msg)?;
            println!(
                "Successfully applied Kubernetes manifests {} URL {}",
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

    // If a directory was provided, apply that next.
    match &manifest.dir {
        Some(dir) => {
            let err_msg = format!(
                "Failed to apply Kubernetes manifests {} using directory {}",
                manifest.name, dir
            );
            // action, name, filename, dry_run
            kubectl_manifests("apply", dir, None, false).context(err_msg)?;
            println!(
                "Successfully applied Kubernetes manifests {} using directory {}",
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
            kubectl_url("delete", url, false).await.context(err_msg)?;
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
            kubectl_manifests("delete", dir, None, false).context(err_msg)?;
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

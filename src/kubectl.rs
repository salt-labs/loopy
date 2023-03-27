//! Kubectl
//!
//! This module contains functions for interacting with Kubernetes using the kubectl CLI.
//!

use crate::utils::run_command;

use anyhow::{Context, Result};
use log::{debug, error, info, info};

//! Applies a single Kubernetes manifest file.
//!
//! # Arguments
//!
//! * `chart_name` - The name of the chart where the manifest file is located
//! * `filename` - The name of the manifest file to apply
//!
//! # Examples
//!
//! ```rust
//! use loopy::kubectl::kubectl_apply_file;
//! fn main() {
//!     let result = kubectl_apply_file("example-chart", "namespace.yaml");
//!     assert!(result.is_ok());
//! }
//! ```
//!
pub fn kubectl_apply_file(chart_name: &str, filename: &str, dry_run: bool) -> Result<()> {
    info!("Applying Kubernetes manifest: {}/{}", chart_name, filename);

    // Construct the manifest file path
    let manifest_path = format!("manifests/{}/{}", chart_name, filename);

    let args = if dry_run {
        vec!["apply", "-f", &manifest_path, "--dry-run=client"]
    } else {
        vec!["apply", "-f", &manifest_path]
    };

    // Apply the manifest file using kubectl
    run_command("kubectl", &args)
        .with_context(|| format!("Failed to apply manifest file: {}", manifest_path))
}

//! Applies all Kubernetes manifests from a specified directory.
//!
//! # Arguments
//!
//! * `chart_name` - The name of the chart where the manifest files are located
//!
//! # Examples
//!
//! ```rust
//! use loopy::kubectl::kubectl_apply_dir;
//! fn main() {
//!     let result = kubectl_apply_dir("example-chart");
//!     assert!(result.is_ok());
//! }
//! ```
//!
pub fn kubectl_apply_dir(chart_name: &str, dry_run: bool) -> Result<()> {
    info!(
        "Applying Kubernetes manifests from directory: {}",
        chart_name
    );

    // Construct the manifest directory path
    let manifest_dir = format!("manifests/{}", chart_name);

    let args = if dry_run {
        vec!["apply", "-f", &manifest_dir, "--dry-run=client"]
    } else {
        vec!["apply", "-f", &manifest_dir]
    };

    // Apply all manifests in the directory using kubectl
    run_command("kubectl", &args)
        .with_context(|| format!("Failed to apply manifests from directory: {}", manifest_dir))
}

//! Deletes all Kubernetes manifests from a specified directory.
//!
//! # Arguments
//!
//! * `chart_name` - The name of the chart where the manifest files are located
//!
//! # Examples
//!
//! ```rust
//! use loopy::kubectl::kubectl_delete_dir;
//! fn main() {
//!     let result = kubectl_delete_dir("example-chart");
//!     assert!(result.is_ok());
//! }
//! ```
//!
pub fn kubectl_delete_dir(chart_name: &str, dry_run: bool) -> Result<()> {
    info!(
        "Deleting Kubernetes manifests from directory: {}",
        chart_name
    );

    // Construct the manifest directory path
    let manifest_dir = format!("manifests/{}", chart_name);

    let args = if dry_run {
        vec!["apply", "-f", &manifest_dir, "--dry-run=client"]
    } else {
        vec!["apply", "-f", &manifest_dir]
    };

    // Delete all manifests in the directory using kubectl
    run_command("kubectl", &args).with_context(|| {
        format!(
            "Failed to delete manifests from directory: {}",
            manifest_dir
        )
    })
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

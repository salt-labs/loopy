//! Defines and validates the configuration file.
//!
//! This module contains the Config struct and its implementation
//! for loading and validating the configuration file.
//!

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

/// Main configuration structure.
#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: Option<Log>,
    pub dependencies: Dependencies,
    pub application: Application,
}

/// Log configuration structure.
#[derive(Debug, Deserialize)]
pub struct Log {
    /// Logging level (e.g., "info", "warn", "error").
    pub level: Option<String>,
    /// Optional log file path.
    pub file: Option<String>,
    /// Boolean to enable or disable showing a fortune cookie.
    /// Defaults to false.
    pub fortune: Option<bool>,
}

/// Dependencies configuration structure.
#[derive(Debug, Deserialize)]
pub struct Dependencies {
    //pub carvel: Carvel,
    pub helm: Helm,
    pub manifests: Vec<Manifests>,
    pub tools: Vec<Tool>,
    pub tests: Vec<Test>,
}

/// Application configuration structure.
#[derive(Debug, Deserialize)]
pub struct Application {
    //pub carvel: Carvel,
    pub helm: Helm,
    pub manifests: Vec<Manifests>,
    pub tests: Vec<Test>,
}

/// Tool configuration structure.
#[derive(Debug, Deserialize)]
pub struct Tool {
    /// Tool name.
    pub name: String,
    /// Tool binary name.
    pub bin: String,
    /// Optional tool URL.
    pub url: Option<String>,
}

/// Test configuration structure.
#[derive(Debug, Deserialize)]
pub struct Test {
    /// Test command.
    pub command: String,
    /// Test arguments.
    pub args: Option<Vec<String>>,
    /// stdout
    pub stdout: Option<String>,
    /// stderr
    pub stderr: Option<String>,
    /// status
    pub status: Option<i32>,
}

/// Manifests configuration structure.
#[derive(Debug, Deserialize)]
pub struct Manifests {
    /// Manifest name.
    pub name: String,
    /// Optional manifest URL.
    pub url: Option<String>,
    /// Optional manifest directory.
    pub dir: Option<String>,
}

/// Carvel configuration structure.
#[derive(Debug, Deserialize)]
pub struct Carvel {
    pub packages: Vec<Package>,
}

/// Package configuration structure for Carvel.
#[derive(Debug, Deserialize)]
pub struct Package {
    /// Package name.
    pub name: String,
    /// Package version.
    pub version: String,
    /// Optional values file for the package.
    pub values: Option<String>,
}

/// Helm configuration structure.
#[derive(Debug, Deserialize)]
pub struct Helm {
    pub repositories: Vec<Repository>,
    pub charts: Vec<Chart>,
}

/// Repository configuration structure for Helm.
#[derive(Debug, Deserialize)]
pub struct Repository {
    /// Repository name.
    pub name: String,
    /// Repository URL.
    pub url: String,
}

/// Chart configuration structure for Helm.
#[derive(Debug, Deserialize)]
pub struct Chart {
    /// The name of the Helm release.
    pub name: String,
    /// The Helm repository where the chart is located.
    pub repo: String,
    /// The Kubernetes namespace in which to deploy the Helm release.
    /// If not provided, defaults to the Helm release name.
    #[serde(default)]
    pub namespace: Option<String>,
    /// The optional name of the values file to use.
    /// If not provided, the default "values.yaml" file is used.
    #[serde(default)]
    pub values: Option<String>,
}

/// Load config.
///
/// Loads the configuration file and perform validation on the file.
///
pub fn load_config(config_file: &str) -> Result<Config> {
    if !Path::new(config_file).exists() {
        let err_msg = format!(
            "The configuration file {} was not found in the current directory.
            You can provide a path to the configuration file with --config",
            config_file
        );
        anyhow::bail!(err_msg);
    }

    // Read and deserialize the config file.
    let err_msg = format!("Failed to read {}", config_file);
    let content = fs::read_to_string(config_file).context(err_msg)?;

    // Process the file to ensure it is valid YAML.
    let err_msg = format!(
        "Failed to parse YAML configuration file. Please ensure the file {} has a valid YAML syntax.",
        config_file,
    );
    let config: Config = serde_yaml::from_str(&content).context(err_msg)?;

    // Perform validation on the config for custom rules.
    validate_config(&config)?;

    Ok(config)
}

/// Validate config.
///
/// Performs validation on the config file.
///
fn validate_config(config: &Config) -> Result<()> {
    // Validate dependencies.tools
    for tool in &config.dependencies.tools {
        // Ensure that the name field of each tool is not empty.
        let err_msg = "The 'name' field of all defined tools cannot be empty.".to_string();
        if tool.name.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // Ensure that the bin field of each tool is not empty.
        let err_msg = format!("The 'bin' field of {} cannot be empty.", tool.name).to_string();
        if tool.bin.trim().is_empty() {
            anyhow::bail!(err_msg);
        }
    }

    // Validate dependencies.tests
    for test in &config.dependencies.tests {
        // Ensure that the command field of each test is not empty.
        let err_msg = "The 'command' field of all defined tests cannot be empty.".to_string();
        if test.command.trim().is_empty() {
            anyhow::bail!(err_msg);
        }
    }

    // Validate dependencies.manifests
    for manifest in &config.dependencies.manifests {
        // Ensure that the name field of each manifest is not empty.
        let err_msg = "The 'name' field of all defined manifests cannot be empty.".to_string();
        if manifest.name.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // Ensure that either the url or dir field of each manifest is not empty.
        let err_msg = format!(
            "The 'url' and 'dir' field of {} cannot both be empty, at least one must be defined.",
            manifest.name
        );
        if manifest.url.is_none() && manifest.dir.is_none() {
            anyhow::bail!(err_msg);
        }
    }

    // Validate dependencies.helm.repositories
    for repo in &config.dependencies.helm.repositories {
        // Ensure that the name field of each repository is not empty.
        let err_msg = "The 'name' field of all defined repositories cannot be empty.".to_string();
        if repo.name.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // Ensure that the url field of each repository is not empty.
        let err_msg = format!("The 'url' field of {} cannot be empty", repo.name).to_string();
        for repo in &config.dependencies.helm.repositories {
            if repo.url.trim().is_empty() {
                anyhow::bail!(err_msg);
            }
        }
    }

    // Validate dependencies.helm.charts
    for chart in &config.dependencies.helm.charts {
        // Ensure that the name field of each chart is not empty.
        let err_msg = "The 'name' field of all defined charts cannot be empty.".to_string();
        if chart.name.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // Ensure that the repo field of each chart is not empty.
        let err_msg =
            format!("The 'repo' field of chart {} cannot be empty.", chart.name).to_string();
        if chart.repo.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // The values field is optional.
    }

    // Validate application.tests
    for test in &config.application.tests {
        // Ensure that the command field of each test is not empty.
        let err_msg = "The 'command' field of all defined tests cannot be empty.".to_string();
        if test.command.trim().is_empty() {
            anyhow::bail!(err_msg);
        }
    }

    // Validate application.manifests
    for manifest in &config.application.manifests {
        // Ensure that the name field of each manifest is not empty.
        let err_msg = "The 'name' field of all defined manifests cannot be empty.".to_string();
        if manifest.name.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // Ensure that either the url or dir field of each manifest is not empty.
        let err_msg = format!(
            "The 'url' and 'dir' field of {} cannot both be empty, at least one must be defined.",
            manifest.name
        );
        if manifest.url.is_none() && manifest.dir.is_none() {
            anyhow::bail!(err_msg);
        }
    }

    // Validate application.helm.repositories
    for repo in &config.application.helm.repositories {
        // Ensure that the name field of each repository is not empty.
        let err_msg = "The 'name' field of all defined repositories cannot be empty.".to_string();
        if repo.name.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // Ensure that the url field of each repository is not empty.
        let err_msg = format!("The 'url' field of {} cannot be empty", repo.name).to_string();
        for repo in &config.application.helm.repositories {
            if repo.url.trim().is_empty() {
                anyhow::bail!(err_msg);
            }
        }
    }

    // Validate application.helm.charts
    for chart in &config.application.helm.charts {
        // Ensure that the name field of each chart is not empty.
        let err_msg = "The 'name' field of all defined charts cannot be empty.".to_string();
        if chart.name.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // Ensure that the repo field of each chart is not empty.
        let err_msg =
            format!("The 'repo' field of chart {} cannot be empty.", chart.name).to_string();
        if chart.repo.trim().is_empty() {
            anyhow::bail!(err_msg);
        }

        // The values field is optional.
    }

    Ok(())
}

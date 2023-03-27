//! Defines and validates the configuration file.
//!
//! This module contains the Config struct and its implementation
//! for loading and validating the configuration file.
//!

use anyhow::{Context, Result};
use serde::Deserialize;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub log: Option<Log>,
    pub dependencies: Dependencies,
    pub application: Application,
}

#[derive(Debug, Deserialize)]
pub struct Log {
    pub level: Option<String>,
    pub file: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Dependencies {
    pub tools: Vec<Tool>,
    pub helm: Helm,
}

#[derive(Debug, Deserialize)]
pub struct Application {
    pub helm: Helm,
}

#[derive(Debug, Deserialize)]
pub struct Tool {
    pub name: String,
    pub bin: String,
    pub url: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Helm {
    pub repositories: Vec<Repository>,
    pub charts: Vec<Chart>,
}

#[derive(Debug, Deserialize)]
pub struct Repository {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize)]
pub struct Chart {
    pub name: String,
    pub repo: String,
}

/// Load config.
///
/// Loads the configuration file and perform validation on the file.
///
pub fn load_config(config_file: &str) -> Result<Config> {
    if !Path::new(config_file).exists() {
        let err_msg = format!(
            "The loopy configuration file {} was not found in the current directory.
            Perhaps you forgot to provide the location with --config",
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
    }

    Ok(())
}

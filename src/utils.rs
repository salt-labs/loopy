//! Utils
//!
//! This module contains utility functions used throughout the program.
//!

use anyhow::{anyhow, Context, Result};
use crossterm::style::{self, Color, Stylize};
use figlet_rs::FIGfont;
use flate2::read::GzDecoder;
use futures_util::StreamExt;
use indicatif::{ProgressBar, ProgressStyle};
use log::{debug, error, info};
use reqwest::Client;
use std::env;
use std::fs::{self, create_dir_all, File};
use std::io::{self, BufReader, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use tar::Archive;
use tempfile::TempDir;
use which::which;
use zip::ZipArchive;

//const GZIP_MAGIC: [u8; 8] = [0x1f, 0x8b, 0x08, 0x00, 0x00, 0x00, 0x00, 0x00];
//const BZIP2_MAGIC: [u8; 3] = [0x42, 0x5a, 0x68];

#[derive(Debug, Clone)]
struct MimeTypes {
    zip: [&'static str; 2],
    tar: [&'static str; 2],
    gzip: [&'static str; 2],
    bzip2: [&'static str; 2],
    bin: [&'static str; 25],
}

const MIME_TYPES: MimeTypes = MimeTypes {
    zip: ["application/zip", "Zip archive data"],
    tar: ["application/x-tar", "POSIX tar archive"],
    gzip: ["application/gzip", "gzip compressed data"],
    bzip2: ["application/x-bzip2", "bzip2 compressed data"],
    bin: [
        "ELF 32-bit LSB core file",
        "ELF 32-bit LSB executable",
        "ELF 32-bit LSB pie executable",
        "ELF 32-bit LSB shared object",
        "ELF 64-bit LSB core file",
        "ELF 64-bit LSB executable",
        "ELF 64-bit LSB pie executable",
        "ELF 64-bit LSB shared object",
        "ELF 64-bit MSB core file",
        "application/octet-stream",
        "application/vnd.android.package-archive",
        "application/vnd.debian.binary-package",
        "application/x-archive",
        "application/x-dosexec",
        "application/x-elf",
        "application/x-executable",
        "application/x-mach-binary",
        "application/x-mach-o",
        "application/x-mach-o-dylib",
        "application/x-mach-o-fat",
        "application/x-mach-o-universal",
        "application/x-msdownload",
        "application/x-object",
        "application/x-pie-executable",
        "application/x-sharedlib",
    ],
};

impl MimeTypes {
    fn all_types(&self) -> [[&'static str; 2]; 4] {
        [self.zip, self.tar, self.gzip, self.bzip2]
    }
}

/// Create directory.
///
/// Creates a directory if it does not already exist.
///
pub fn create_dir(dir: &PathBuf) -> Result<()> {
    if dir.exists() {
        info!("Directory already exists: {:?}", dir);
        return Ok(());
    }

    match fs::create_dir_all(dir) {
        Ok(_) => {
            info!("Created directory: {:?}", dir);
            Ok(())
        }
        Err(e) => {
            error!("Failed to create directory: {:?} - {}", dir, e);
            Err(e.into())
        }
    }
}

/// figlet
///
/// Use figlet in Rust.
///
pub fn figlet(
    input: &str,
    header_color: Option<Color>,
    footer_color: Option<Color>,
    figure_color: Option<Color>,
) {
    let font_result = FIGfont::standard();

    match font_result {
        Ok(font) => {
            let figure_option = font.convert(input);
            let header = "##################################################\n";
            let footer = "##################################################\n";

            if let Some(figure) = figure_option {
                // Check if terminal supports colors first.
                if crossterm::terminal::size().is_ok() {
                    // Set the color of the header.
                    let colored_header = style::style(header)
                        .stylize()
                        .with(header_color.unwrap_or(Color::Yellow))
                        //.on(Color::Black)
                        .bold();

                    // Set the color of the footer.
                    let colored_footer = style::style(footer)
                        .stylize()
                        .with(footer_color.unwrap_or(Color::Yellow))
                        //.on(Color::Black)
                        .bold();

                    // Set the color of the figure.
                    let colored_figure = style::style(figure)
                        .stylize()
                        .with(figure_color.unwrap_or(Color::Green))
                        //.on(Color::Black)
                        .bold();

                    println!("{}", colored_header);
                    println!("{}", colored_figure);
                    println!("{}", colored_footer);
                } else {
                    let figure = style::style(figure).stylize();

                    println!("{}", header);
                    println!("{}", figure);
                    println!("{}", footer);
                };
            } else {
                eprintln!("Failed to convert input to ASCII art");
            }
        }
        Err(e) => {
            eprintln!("Failed to load standard font: {}", e);
        }
    }
}

/// Update PATH
///
/// Updates the PATH environment variable to include the vendor directory
///
pub fn update_path(dir: &PathBuf) {
    // Get the full path of the provided directory.
    let dir = match fs::canonicalize(dir) {
        Ok(dir) => {
            debug!("Canonical path: {:?}", dir);
            dir
        }
        Err(e) => {
            error!("Failed to get canonical path of {:?} - {}", dir, e);
            return;
        }
    };

    // Get the current PATH environment variable value.
    let mut paths = match env::var_os("PATH") {
        Some(paths) => env::split_paths(&paths).collect(),
        None => Vec::new(),
    };

    // Add the provided directory to the PATH if it does not already exist.
    if !paths.contains(&dir) {
        debug! {"Adding {:?} to PATH", &dir};
        paths.push(dir);
    } else {
        debug!("Directory already exists in PATH: {:?}", dir);
    }

    // Update the PATH environment variable.
    let new_path = env::join_paths(paths).expect("Failed to join PATHs");
    env::set_var("PATH", new_path);

    debug!("PATH: {:?}", env::var_os("PATH"));
}

/// Detects if a given path is an archive.
///
/// # Arguments
///
/// * `path` - A path to a file
///
/// # Returns
///
/// A tuple with a boolean indicating if the file is an archive and the mime type of the file.
/// Detects if a given path is an archive.
///
/// # Arguments
///
/// * `path` - A path to a file
///
/// # Returns
///
/// A tuple with a boolean indicating if the file is an archive and the mime type of the file.
///
fn detect_archive(path: &Path) -> Result<(bool, String)> {
    let cookie =
        magic::Cookie::open(magic::CookieFlags::ERROR).context("Failed to open magic Cookie")?;
    cookie
        .load::<&str>(&[])
        .context("Failed to load magic Cookie")?;

    let mut file = std::fs::File::open(path).context(format!("Failed to open file: {:?}", path))?;
    let mut buffer = [0; 1024];
    let count = file.read(&mut buffer).context("Failed to read file")?;
    let mime_type = cookie
        .buffer(&buffer[..count])
        .context("Failed to get mime type")?;

    debug!(
        "Checking for mime type: {} in all defined archive types: {:?}",
        mime_type, MIME_TYPES
    );

    if MIME_TYPES
        .all_types()
        .iter()
        .flatten()
        .any(|&t| mime_type.starts_with(t))
    {
        file.seek(SeekFrom::Start(0))?;
        let mut buf = [0u8; 8];
        file.read_exact(&mut buf)?;
        let is_archive: bool = true;
        debug!(
            "An archive of mime type {} was detected: {}",
            mime_type,
            &path.display()
        );
        Ok((is_archive, mime_type))
    } else {
        debug!(
            "A mime type of {} was detected, this is not an archive: {}",
            mime_type,
            &path.display()
        );
        Ok((false, mime_type))
    }
}

/// Detect Binary
///
/// Detects if the file at the given path is a binary.
///
/// This is done by checking the mime type of the file.
///
/// If the mime type is in the `MIME_TYPES.bin` array, then the file is a binary.
/// This is not a perfect solution, but it works for the most part.
///
pub fn detect_binary(path: &Path) -> Result<bool> {
    let cookie =
        magic::Cookie::open(magic::CookieFlags::ERROR).context("Failed to open magic Cookie")?;
    cookie
        .load::<&str>(&[])
        .context("Failed to load magic Cookie")?;

    let mut file = std::fs::File::open(path).context(format!("Failed to open file: {:?}", path))?;
    let mut buffer = [0; 1024];
    let count = file.read(&mut buffer).context("Failed to read file")?;
    let mime_type = cookie
        .buffer(&buffer[..count])
        .context("Failed to get mime type")?;
    debug!(
        "The Mime type on path: {} is: {}",
        path.display(),
        mime_type
    );

    debug!(
        "Checking for mime type: {} in all defined archive types: {:?}",
        mime_type, MIME_TYPES.bin
    );

    Ok(MIME_TYPES.bin.iter().any(|&t| mime_type.starts_with(t)))
}

/// Extract Archive.
///
/// Extracts a provided archive file from common archive formats.
///
/// # Arguments
///
/// * `path` - A path to an archive
/// * `dest` - A path to the destination directory
///
/// # Returns
///
/// A `Result` containing the path to the extracted archive if successful, or an error if the extraction fails
///
fn extract_archive(path: &Path, dest: &Path) -> Result<PathBuf> {
    // Check if the provided path is an archive.
    let (is_archive, mime_type) = detect_archive(path)?;

    // If the provided path is not an archive, return an error.
    if !is_archive {
        return Err(anyhow!("path {} is not an archive", path.display()));
    }

    let file = File::open(path).context(format!("Failed to open archive file: {:?}", path))?;

    // Extract the archive based on the mime type.
    let extracted_archive_path = match mime_type.as_str() {
        // Extract ZIP files
        archive_type if MIME_TYPES.zip.iter().any(|t| archive_type.starts_with(t)) => {
            info!("Extracting zip archive: {}", &path.display());
            extract_archive_zip(file, dest).context("Failed to extract zip archive")?
        }

        // Extract GZIP files
        archive_type if MIME_TYPES.gzip.iter().any(|t| archive_type.starts_with(t)) => {
            info!("Extracting gzip archive: {}", &path.display());
            extract_archive_gzip(file, dest).context("Failed to extract gzip archive")?
        }

        // Extract TAR files
        archive_type if MIME_TYPES.tar.iter().any(|t| archive_type.starts_with(t)) => {
            info!("Extracting tar archive: {}", &path.display());
            extract_archive_tar(file, dest).context("Failed to extract tar archive")?
        }

        // Extract BZIP2 files
        archive_type if MIME_TYPES.bzip2.iter().any(|t| archive_type.starts_with(t)) => {
            info!("Extracting bzip2 archive: {}", &path.display());
            extract_archive_bzip2(file, dest).context("Failed to extract bzip2 archive")?
        }

        _ => {
            return Err(io::Error::new(
                io::ErrorKind::Other,
                format!(
                    "Extracting the archive format {} is not yet implemented",
                    mime_type
                ),
            )
            .into())
        }
    };

    Ok(extracted_archive_path)
}

/// Extract Archive ZIP
///
/// Extracts a ZIP archive to the specified destination directory.
///
/// # Arguments
///
/// * `file` - A `File` containing the ZIP archive
/// * `dest` - The destination directory to extract the archive to
///
/// # Returns
///
/// A `Result` containing the path to the extracted archive if successful, or an error if the extraction fails
///
/// A `Result` containing the path to the extracted archive if successful, or an error if the extraction fails
///
fn extract_archive_zip(file: File, dest: &Path) -> Result<PathBuf> {
    let mut archive = ZipArchive::new(BufReader::new(file))?;

    for i in 0..archive.len() {
        let mut file = archive.by_index(i)?;
        let outpath = dest.join(file.name());
        if file.is_dir() {
            create_dir_all(&outpath)?;
        } else {
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    create_dir_all(p)?;
                }
            }
            let mut outfile = File::create(&outpath)?;
            std::io::copy(&mut file, &mut outfile)?;
        }
    }

    Ok(dest.to_owned())
}

/// Extract Archive GZIP
///
/// Extracts a GZIP archive to the specified destination directory.
///
/// # Arguments
///
/// * `file` - A `File` containing the GZIP archive
/// * `dest` - The destination directory to extract the archive to
///
/// # Returns
///
/// A `Result` containing the path to the extracted archive if successful, or an error if the extraction fails
///
fn extract_archive_gzip(file: File, dest: &Path) -> Result<PathBuf> {
    let decoder = GzDecoder::new(file);
    let mut archive = Archive::new(decoder);
    archive.unpack(dest)?;

    Ok(dest.to_owned())
}

/// Extract Archive Tar
///
/// Extracts a TAR archive to the specified destination directory.
///
/// # Arguments
///
/// * `file` - A `File` containing the TAR archive
/// * `dest` - The destination directory to extract the archive to
///
/// # Returns
///
/// A `Result` containing the path to the extracted archive if successful, or an error if the extraction fails
///
fn extract_archive_tar(file: File, dest: &Path) -> Result<PathBuf> {
    let mut archive = Archive::new(BufReader::new(file));
    archive.unpack(dest)?;

    Ok(dest.to_owned())
}

/// Extract Archive BZIP2
///
/// Extracts a BZIP2 archive to the specified destination directory.
///
/// # Arguments
///
/// * `file` - A `File` containing the BZIP2 archive
/// * `dest` - The destination directory to extract the archive to
///
/// # Returns
///
/// A `Result` containing the path to the extracted archive if successful, or an error if the extraction fails
///
fn extract_archive_bzip2(file: File, dest: &Path) -> Result<PathBuf> {
    let mut archive = Archive::new(BufReader::new(file));
    archive.unpack(dest)?;

    Ok(dest.to_owned())
}

/// Search Archive.
///
/// Search a given path for a file matching a provided name, and return its path if it is a binary.
///
/// If no binary or more than one binary is found matching the name, return an error.
///
pub fn search_archive(path: &Path, name: &str) -> Result<PathBuf> {
    let mut binary_path: Option<PathBuf> = None;
    search_helper(path, name, &mut binary_path)?;
    binary_path.ok_or_else(|| anyhow!("Failed to find a binary matching '{}'", name))
}

/// Search Archive Helper
///
/// Helper function for search_archive.
///
fn search_helper(path: &Path, name: &str, binary_path: &mut Option<PathBuf>) -> Result<()> {
    for entry in
        std::fs::read_dir(path).with_context(|| format!("Failed to read directory: {:?}", path))?
    {
        let entry = entry.with_context(|| format!("Failed to get directory entry: {:?}", path))?;
        let metadata = entry.metadata()?;
        let file_type = metadata.file_type();
        let file_path = entry.path();

        if file_type.is_dir() {
            search_helper(&file_path, name, binary_path)?;
        } else {
            let file_name = entry.file_name().to_string_lossy().to_string();

            // If the file name contains the name of the binary we're looking for
            if file_name.contains(name) {
                // If the file is a binary, return its path
                if detect_binary(&file_path)
                    .with_context(|| format!("Failed to detect binary: {:?}", file_path))?
                {
                    debug!("Found binary: {}", file_path.display());
                    if let Some(existing_path) = binary_path {
                        return Err(anyhow!(
                            "Found multiple binaries matching '{}':\n{}\n{}",
                            name,
                            existing_path.display(),
                            file_path.display()
                        ));
                    }

                    *binary_path = Some(file_path);
                } else {
                    debug!(
                        "Found matching file name that wasn't a binary file: {}",
                        file_path.display()
                    );
                }
            } else {
                debug!("Skipping non-matching file: {}", file_name);
            }
        }
    }

    Ok(())
}

/// Check command in PATH
///
/// Checks for a given command in the PATH using the `which` crate.
///
/// # Arguments
///
/// * `command` - The command to check for in the PATH
///
/// # Returns
///
/// A `Result` containing `()` if the command is found in the PATH, or an error if the command is not found
///
pub fn check_command_in_path(command: &str) -> Result<(), ()> {
    debug!("Checking if command '{}' is in PATH", command);
    match which(command) {
        Ok(path) => {
            debug!("Found command '{}' at path: {:?}", command, path);
            Ok(())
        }
        Err(_) => {
            debug!("Command '{}' not found in PATH", command);
            Err(())
        }
    }
}

/// Download Tool.
///
/// Downloads the specified tool using the URL from the configuration file.
///
/// # Arguments
///
/// * `client` - A reference to the `Client` struct for making HTTP requests
/// * `url` - A string slice containing the URL to download the tool from
/// * `name` - The name of the binary to download
/// * `vendor_dir` - A reference to the vendor directory to store the downloaded binary
///
/// # Returns
///
/// A `Result` containing the path of the downloaded binary if successful, or an error if the download fails
///
pub async fn download_tool(
    client: &Client,
    url: &str,
    name: &str,
    vendor_dir: &Path,
) -> Result<PathBuf> {
    let res = client
        .get(url)
        .send()
        .await
        .context(format!("Failed to GET from '{}'", url))?;
    let total_size = res
        .content_length()
        .ok_or_else(|| anyhow!("Failed to get content length from '{}'", url))?;

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{msg} {bar:40.cyan/blue} {pos}/{len} {bytes_per_sec}")
            .unwrap(),
    );

    // Set the download and downloaded messages
    let download_message: &'static str = "Downloading";
    let downloaded_message: &'static str = "Downloaded";

    // Create a temporary directory to download the tools into.
    // This get automatically deleted when the `TempDir` struct goes out of scope.
    let temp_dir = TempDir::new()?;

    // Create the vendor directory if it does not already exist.
    create_dir(&vendor_dir.to_path_buf())?;

    // Define where the tool will be downloaded to and where it will be moved to.
    let tool_download_path = temp_dir.path().join(name);
    let tool_vendor_path = vendor_dir.join(name);

    debug!("Tool download path: {}", tool_download_path.display());
    debug!("Tool vendor path: {}", tool_vendor_path.display());

    let mut file =
        File::create(&tool_download_path).context("Failed to create temporary file".to_string())?;

    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(chunk_result) = stream.next().await {
        let chunk = chunk_result.context("Error while reading response body")?;
        let size = chunk.len();

        file.write_all(&chunk)
            .context("Failed to write to temporary file")?;

        downloaded += size as u64;
        pb.set_message(download_message);
        pb.set_position(downloaded);
    }
    pb.finish_with_message(downloaded_message);

    // Default to assuming the download was the binary itself.
    let mut binary_path = tool_download_path.clone();

    // If the download was an archive file, extract it.
    if let Ok((is_archive, _mime_type)) = detect_archive(&tool_download_path) {
        if is_archive {
            // Set the extraction directory name to have a suffix of "-extracted"
            let dest_dir = temp_dir.path().join(format!("{}-extracted", name));
            // Extract the archive and capture the path to the extracted archive.
            let extracted_archive = extract_archive(&tool_download_path, &dest_dir)
                .map_err(|e| {
                    error!("Failed to extract archive: {}", e);
                    e
                })
                .context("Failed to extract archive. Review the log for further information.")?;
            info!("Extracted archive to {}", extracted_archive.display());
            // Search the extracted archive for the binary
            binary_path = search_archive(&extracted_archive, name)?;
        }
    } else {
        debug!("Not an archive file: {}", tool_download_path.display());
    }

    // Copy the downloaded binary to the vendor directory
    let target_path = vendor_dir.join(name);
    debug!(
        "Copying binary {} to vendor directory: {}",
        binary_path.display(),
        target_path.display()
    );
    std::fs::copy(&binary_path, &target_path)
        .context("Failed to move binary to vendor directory".to_string())?;

    Ok(target_path)
}

/// Run Command.
///
/// Runs a command with the provided arguments
///
/// # Arguments
///
/// * `cmd_name` - The name of the command to run
/// * `args` - A slice of string references representing the arguments to pass to the command
///
/// # Returns
///
/// A `Result` containing the output of the command if successful, or an error if the command fails
///
pub fn run_command(cmd_name: &str, args: &[&str]) -> Result<String> {
    // Check if the command exists in the path.
    if check_command_in_path(cmd_name).is_err() {
        return Err(anyhow::anyhow!("Command '{}' not found in PATH", cmd_name));
    }

    let mut cmd = Command::new(cmd_name);

    // Add command arguments if they exist.
    if !args.is_empty() {
        cmd.args(args);
    }

    let output = cmd
        .output()
        .with_context(|| format!("Failed to execute command '{}'", cmd_name))?;

    if output.status.success() {
        println!("Command '{}' succeeded", cmd_name);
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    } else {
        Err(anyhow::anyhow!(
            "Command '{}' failed with error: {}",
            cmd_name,
            String::from_utf8_lossy(&output.stderr)
        ))
    }
}

use crate::error::{OptimizerError, OptimizerResult};
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{Duration, Instant};

/// Print a status message with colored output
pub fn print_status(message: &str) {
    println!("{} {}", "[INFO]".bright_blue().bold(), message);
}

/// Print a success message with colored output
pub fn print_success(message: &str) {
    println!("{} {}", "[SUCCESS]".bright_green().bold(), message);
}

/// Print a warning message with colored output
pub fn print_warning(message: &str) {
    println!("{} {}", "[WARNING]".bright_yellow().bold(), message);
}

/// Print an error message with colored output
pub fn print_error(message: &str) {
    eprintln!("{} {}", "[ERROR]".bright_red().bold(), message);
}

/// Check if we're in a Rust project directory
pub fn is_rust_project<P: AsRef<Path>>(path: P) -> bool {
    path.as_ref().join("Cargo.toml").exists()
}

/// Find the root of a Rust project by walking up the directory tree
pub fn find_rust_project_root<P: AsRef<Path>>(start_path: P) -> OptimizerResult<PathBuf> {
    let mut current = start_path.as_ref().to_path_buf();
    
    loop {
        if current.join("Cargo.toml").exists() {
            return Ok(current);
        }
        
        if let Some(parent) = current.parent() {
            current = parent.to_path_buf();
        } else {
            return Err(OptimizerError::project_validation(
                "No Cargo.toml found in current directory or any parent directory"
            ));
        }
    }
}

/// Execute a command and return the result
pub fn execute_command(command: &str, args: &[&str], working_dir: Option<&Path>) -> OptimizerResult<Output> {
    let mut cmd = Command::new(command);
    cmd.args(args);
    
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }
    
    let output = cmd.output()
        .map_err(|e| OptimizerError::command_failed(format!("Failed to execute {}: {}", command, e)))?;
    
    Ok(output)
}

/// Execute a command and return success status
pub fn execute_command_success(command: &str, args: &[&str], working_dir: Option<&Path>) -> OptimizerResult<bool> {
    let output = execute_command(command, args, working_dir)?;
    Ok(output.status.success())
}

/// Execute a command with real-time output
pub fn execute_command_with_output(
    command: &str, 
    args: &[&str], 
    working_dir: Option<&Path>
) -> OptimizerResult<()> {
    let mut cmd = Command::new(command);
    cmd.args(args);
    
    if let Some(dir) = working_dir {
        cmd.current_dir(dir);
    }
    
    let status = cmd.status()
        .map_err(|e| OptimizerError::command_failed(format!("Failed to execute {}: {}", command, e)))?;
    
    if !status.success() {
        return Err(OptimizerError::command_failed(
            format!("Command {} failed with exit code: {:?}", command, status.code())
        ));
    }
    
    Ok(())
}

/// Create a progress bar with a custom style
pub fn create_progress_bar(len: u64, message: &str) -> ProgressBar {
    let pb = ProgressBar::new(len);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-")
    );
    pb.set_message(message.to_string());
    pb
}

/// Create an indeterminate progress spinner
pub fn create_spinner(message: &str) -> ProgressBar {
    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.green} {msg}")
            .unwrap()
    );
    pb.set_message(message.to_string());
    pb.enable_steady_tick(Duration::from_millis(100));
    pb
}

/// Backup a file by copying it with a .backup extension
pub fn backup_file<P: AsRef<Path>>(path: P) -> OptimizerResult<PathBuf> {
    let original = path.as_ref();
    if !original.exists() {
        return Err(OptimizerError::file_not_found(original.display().to_string()));
    }
    
    let backup_path = original.with_extension(
        format!("{}.backup", 
            original.extension()
                .and_then(|ext| ext.to_str())
                .unwrap_or("")
        )
    );
    
    fs::copy(original, &backup_path)?;
    print_status(&format!("Backed up {} to {}", 
        original.display(), 
        backup_path.display()
    ));
    
    Ok(backup_path)
}

/// Get the size of a directory in bytes
pub fn get_directory_size<P: AsRef<Path>>(path: P) -> OptimizerResult<u64> {
    let mut size = 0;
    
    for entry in walkdir::WalkDir::new(path) {
        let entry = entry.map_err(|e| OptimizerError::Io(e.into()))?;
        if entry.file_type().is_file() {
            size += entry.metadata()
                .map_err(|e| OptimizerError::Io(e.into()))?
                .len();
        }
    }
    
    Ok(size)
}

/// Format bytes into a human-readable string
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

/// Format duration into a human-readable string
pub fn format_duration(duration: Duration) -> String {
    let total_seconds = duration.as_secs();
    let minutes = total_seconds / 60;
    let seconds = total_seconds % 60;
    let millis = duration.subsec_millis();
    
    if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else if seconds > 0 {
        format!("{}.{}s", seconds, millis / 100)
    } else {
        format!("{}ms", millis)
    }
}

/// Measure execution time of a function
pub fn measure_time<F, R>(f: F) -> (R, Duration)
where
    F: FnOnce() -> R,
{
    let start = Instant::now();
    let result = f();
    let duration = start.elapsed();
    (result, duration)
}

/// Check if a tool is available in PATH
pub fn is_tool_available(tool: &str) -> bool {
    which::which(tool).is_ok()
}

/// Get the version of a tool
pub fn get_tool_version(tool: &str) -> Option<String> {
    let output = Command::new(tool)
        .arg("--version")
        .output()
        .ok()?;
    
    if output.status.success() {
        String::from_utf8(output.stdout)
            .ok()
            .and_then(|s| s.lines().next().map(|line| line.trim().to_string()))
    } else {
        None
    }
}

/// Clean old files in a directory based on age
pub fn clean_old_files<P: AsRef<Path>>(
    directory: P, 
    max_age_days: u32,
    pattern: Option<&str>
) -> OptimizerResult<u64> {
    let max_age = Duration::from_secs(max_age_days as u64 * 24 * 60 * 60);
    let now = std::time::SystemTime::now();
    let mut cleaned_size = 0u64;
    
    for entry in walkdir::WalkDir::new(directory) {
        let entry = entry.map_err(|e| OptimizerError::Io(e.into()))?;
        
        if !entry.file_type().is_file() {
            continue;
        }
        
        // Check pattern if provided
        if let Some(pattern) = pattern {
            if !entry.file_name().to_string_lossy().contains(pattern) {
                continue;
            }
        }
        
        let metadata = entry.metadata().map_err(|e| OptimizerError::Io(e.into()))?;
        let modified = metadata.modified().map_err(|e| OptimizerError::Io(e.into()))?;
        
        if let Ok(age) = now.duration_since(modified) {
            if age > max_age {
                let size = metadata.len();
                if fs::remove_file(entry.path()).is_ok() {
                    cleaned_size += size;
                    log::debug!("Cleaned old file: {}", entry.path().display());
                }
            }
        }
    }
    
    Ok(cleaned_size)
}

/// Confirm an action with the user
pub fn confirm(message: &str) -> OptimizerResult<bool> {
    use dialoguer::Confirm;
    
    Confirm::new()
        .with_prompt(message)
        .default(false)
        .interact()
        .map_err(|_| OptimizerError::Cancelled)
}

/// Select from a list of options
pub fn select_from_list<T: ToString>(
    message: &str, 
    items: &[T]
) -> OptimizerResult<usize> {
    use dialoguer::Select;
    
    let items: Vec<String> = items.iter().map(|item| item.to_string()).collect();
    
    Select::new()
        .with_prompt(message)
        .items(&items)
        .default(0)
        .interact()
        .map_err(|_| OptimizerError::Cancelled)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(512), "512 B");
        assert_eq!(format_bytes(1024), "1.0 KB");
        assert_eq!(format_bytes(1536), "1.5 KB");
        assert_eq!(format_bytes(1048576), "1.0 MB");
    }
    
    #[test]
    fn test_format_duration() {
        assert_eq!(format_duration(Duration::from_millis(500)), "500ms");
        assert_eq!(format_duration(Duration::from_secs(1)), "1.0s");
        assert_eq!(format_duration(Duration::from_secs(65)), "1m 5s");
    }
}

use crate::config::{generate_cargo_config, generate_cargo_profiles, OptimizerConfig};
use crate::error::{OptimizerError, OptimizerResult};
use crate::system::SystemInfo;
use crate::utils::*;
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};

pub async fn run(
    project_dir: Option<PathBuf>,
    no_backup: bool,
    no_tools: bool,
    force: bool,
) -> OptimizerResult<()> {
    let project_root = if let Some(dir) = project_dir {
        dir
    } else {
        find_rust_project_root(".")?
    };

    print_status(&format!(
        "Initializing optimization for project: {}",
        project_root.display()
    ));

    // Validate that this is a Rust project
    if !is_rust_project(&project_root) {
        return Err(OptimizerError::project_validation(
            "No Cargo.toml found. Please run this command from a Rust project directory.",
        ));
    }

    // Detect system information
    let system_info = SystemInfo::detect()?;
    print_status(&format!(
        "Detected system: {} {} with {} CPU cores",
        system_info.os, system_info.arch, system_info.cpu_cores
    ));

    // Load or create configuration
    let config = OptimizerConfig::load_or_default()?;
    config.validate()?;

    // Backup existing files if requested
    if !no_backup {
        backup_existing_files(&project_root)?;
    }

    // Install configuration files
    install_cargo_config(&project_root, &config, &system_info, force)?;
    install_cargo_profiles(&project_root, force)?;

    // Install tools if requested
    if !no_tools {
        print_status("Installing required optimization tools...");
        crate::commands::tools::install_tools(&config.tools.preferred_tools).await?;
    }

    // Create optimization scripts directory
    create_scripts_directory(&project_root)?;

    // Save configuration
    OptimizerConfig::save_default()?;

    print_success("🎉 Rust build optimization initialized successfully!");
    print_next_steps();

    Ok(())
}

fn backup_existing_files(project_root: &Path) -> OptimizerResult<()> {
    print_status("Backing up existing files...");

    let cargo_config_path = project_root.join(".cargo").join("config.toml");
    if cargo_config_path.exists() {
        backup_file(&cargo_config_path)?;
    }

    let cargo_toml_path = project_root.join("Cargo.toml");
    if cargo_toml_path.exists() {
        backup_file(&cargo_toml_path)?;
    }

    Ok(())
}

fn install_cargo_config(
    project_root: &Path,
    config: &OptimizerConfig,
    system_info: &SystemInfo,
    force: bool,
) -> OptimizerResult<()> {
    let cargo_dir = project_root.join(".cargo");
    let config_path = cargo_dir.join("config.toml");

    // Create .cargo directory if it doesn't exist
    fs::create_dir_all(&cargo_dir)?;

    // Check if config already exists
    if config_path.exists() && !force {
        if !confirm("Cargo config already exists. Overwrite?")? {
            print_warning("Skipping Cargo config installation");
            return Ok(());
        }
    }

    // Generate optimized Cargo configuration
    let config_content = generate_cargo_config(config, system_info);

    // Write configuration file
    fs::write(&config_path, config_content)?;
    print_success(&format!(
        "Installed optimized Cargo config: {}",
        config_path.display()
    ));

    Ok(())
}

fn install_cargo_profiles(project_root: &Path, force: bool) -> OptimizerResult<()> {
    let cargo_toml_path = project_root.join("Cargo.toml");

    if !cargo_toml_path.exists() {
        return Err(OptimizerError::file_not_found("Cargo.toml"));
    }

    // Read existing Cargo.toml
    let existing_content = fs::read_to_string(&cargo_toml_path)?;

    // Check if profiles already exist
    if existing_content.contains("[profile.dev]") && !force {
        if !confirm("Cargo.toml already contains profiles. Add optimized profiles anyway?")? {
            print_warning("Skipping Cargo.toml profile optimization");
            return Ok(());
        }
    }

    // Generate optimized profiles
    let profiles_content = generate_cargo_profiles();

    // Append profiles to Cargo.toml
    let mut new_content = existing_content;
    if !new_content.ends_with('\n') {
        new_content.push('\n');
    }
    new_content.push('\n');
    new_content.push_str("# Optimized build profiles added by Atlas\n");
    new_content.push_str(&profiles_content);

    // Write updated Cargo.toml
    fs::write(&cargo_toml_path, new_content)?;
    print_success("Added optimized build profiles to Cargo.toml");

    Ok(())
}

fn create_scripts_directory(project_root: &Path) -> OptimizerResult<()> {
    let scripts_dir = project_root.join("scripts");

    if !scripts_dir.exists() {
        fs::create_dir_all(&scripts_dir)?;
        print_status(&format!(
            "Created scripts directory: {}",
            scripts_dir.display()
        ));
    }

    // Create a simple build script as an example
    let build_script_path = scripts_dir.join("fast-build.sh");
    if !build_script_path.exists() {
        let build_script_content = r#"#!/bin/bash
# Fast build script generated by Atlas
# Use Atlas for more advanced features

set -e

case "${1:-check}" in
    "check")
        echo "Running fast cargo check..."
        cargo check --workspace --all-targets
        ;;
    "build")
        echo "Running optimized cargo build..."
        cargo build --workspace
        ;;
    "test")
        echo "Running fast tests..."
        if command -v cargo-nextest >/dev/null 2>&1; then
            cargo nextest run --workspace
        else
            cargo test --workspace
        fi
        ;;
    "release")
        echo "Running optimized release build..."
        cargo build --workspace --release
        ;;
    *)
        echo "Usage: $0 [check|build|test|release]"
        exit 1
        ;;
esac
"#;
        fs::write(&build_script_path, build_script_content)?;

        // Make script executable on Unix systems
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let mut perms = fs::metadata(&build_script_path)?.permissions();
            perms.set_mode(0o755);
            fs::set_permissions(&build_script_path, perms)?;
        }

        print_success(&format!(
            "Created build script: {}",
            build_script_path.display()
        ));
    }

    Ok(())
}

fn print_next_steps() {
    println!();
    print_success("🎉 Rust Build Optimization initialized successfully!");
    println!();
    println!("📋 Next steps:");
    println!(
        "   1. Test the optimizations: {}",
        "atlas build check".bright_green()
    );
    println!(
        "   2. Run development workflow: {}",
        "atlas dev watch".bright_green()
    );
    println!(
        "   3. Check optimization status: {}",
        "atlas status".bright_green()
    );
    println!(
        "   4. View configuration: {}",
        "atlas config show".bright_green()
    );
    println!();
    println!("🚀 Quick commands:");
    println!(
        "   {} - Fast syntax check",
        "atlas dev quick-check".bright_cyan()
    );
    println!(
        "   {} - Optimized build",
        "atlas build build".bright_cyan()
    );
    println!(
        "   {} - Fast testing",
        "atlas build test".bright_cyan()
    );
    println!(
        "   {} - Continuous development",
        "atlas dev watch".bright_cyan()
    );
    println!();
    println!(
        "📚 For help: {}",
        "atlas --help".bright_yellow()
    );
}

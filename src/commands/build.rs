use crate::error::{OptimizerError, OptimizerResult};
use crate::utils::*;
use crate::BuildCommands;
use std::path::PathBuf;
use std::time::Instant;

pub async fn run(build_type: BuildCommands, project_dir: Option<PathBuf>) -> OptimizerResult<()> {
    let project_root = if let Some(dir) = project_dir {
        dir
    } else {
        find_rust_project_root(".")?
    };

    // Validate that this is a Rust project
    if !is_rust_project(&project_root) {
        return Err(OptimizerError::project_validation(
            "No Cargo.toml found. Please run this command from a Rust project directory.",
        ));
    }

    match build_type {
        BuildCommands::Check { stats } => run_check(&project_root, stats).await,
        BuildCommands::Build { release, stats } => run_build(&project_root, release, stats).await,
        BuildCommands::Test { changed, stats } => run_test(&project_root, changed, stats).await,
        BuildCommands::Clean { all } => run_clean(&project_root, all).await,
    }
}

async fn run_check(project_root: &PathBuf, show_stats: bool) -> OptimizerResult<()> {
    print_status("Running optimized cargo check...");

    let start_time = Instant::now();

    let result = execute_command_with_output(
        "cargo",
        &["check", "--workspace", "--all-targets"],
        Some(project_root),
    );

    let duration = start_time.elapsed();

    match result {
        Ok(_) => {
            print_success(&format!(
                "✅ Check completed in {}",
                format_duration(duration)
            ));

            if show_stats {
                show_build_stats(project_root, "check", duration).await?;
            }

            Ok(())
        }
        Err(e) => {
            print_error(&format!("❌ Check failed: {}", e));
            Err(e)
        }
    }
}

async fn run_build(project_root: &PathBuf, release: bool, show_stats: bool) -> OptimizerResult<()> {
    let build_type = if release { "release" } else { "debug" };
    print_status(&format!(
        "Running optimized cargo build ({})...",
        build_type
    ));

    let start_time = Instant::now();

    let mut args = vec!["build", "--workspace"];
    if release {
        args.push("--release");
    }

    let result = execute_command_with_output("cargo", &args, Some(project_root));
    let duration = start_time.elapsed();

    match result {
        Ok(_) => {
            print_success(&format!(
                "✅ Build completed in {}",
                format_duration(duration)
            ));

            if show_stats {
                show_build_stats(project_root, build_type, duration).await?;
            }

            Ok(())
        }
        Err(e) => {
            print_error(&format!("❌ Build failed: {}", e));
            Err(e)
        }
    }
}

async fn run_test(project_root: &PathBuf, _changed: bool, show_stats: bool) -> OptimizerResult<()> {
    print_status("Running optimized tests...");

    let start_time = Instant::now();

    // Try to use cargo-nextest if available, otherwise fall back to cargo test
    let result = if is_tool_available("cargo-nextest") {
        print_status("Using cargo-nextest for faster testing...");
        execute_command_with_output(
            "cargo",
            &["nextest", "run", "--workspace"],
            Some(project_root),
        )
    } else {
        execute_command_with_output("cargo", &["test", "--workspace"], Some(project_root))
    };

    let duration = start_time.elapsed();

    match result {
        Ok(_) => {
            print_success(&format!(
                "✅ Tests completed in {}",
                format_duration(duration)
            ));

            if show_stats {
                show_build_stats(project_root, "test", duration).await?;
            }

            Ok(())
        }
        Err(e) => {
            print_error(&format!("❌ Tests failed: {}", e));
            Err(e)
        }
    }
}

async fn run_clean(project_root: &PathBuf, all: bool) -> OptimizerResult<()> {
    print_status("Cleaning build artifacts...");

    if all {
        // Clean everything including rust-analyzer cache
        execute_command_with_output("cargo", &["clean"], Some(project_root))?;

        let rust_analyzer_target = project_root.join("target").join("rust-analyzer");
        if rust_analyzer_target.exists() {
            std::fs::remove_dir_all(&rust_analyzer_target)?;
            print_status("Cleaned rust-analyzer cache");
        }

        print_success("✅ Complete clean finished");
    } else {
        // Selective clean to preserve incremental compilation
        let target_dir = project_root.join("target");
        if target_dir.exists() {
            // Clean only specific artifacts
            let debug_dir = target_dir.join("debug");
            if debug_dir.exists() {
                let deps_dir = debug_dir.join("deps");
                if deps_dir.exists() {
                    // Remove .rlib files but keep incremental data
                    for entry in std::fs::read_dir(&deps_dir)? {
                        let entry = entry?;
                        if let Some(ext) = entry.path().extension() {
                            if ext == "rlib" {
                                std::fs::remove_file(entry.path())?;
                            }
                        }
                    }
                }
            }
        }

        print_success("✅ Selective clean finished (preserved incremental compilation data)");
    }

    Ok(())
}

async fn show_build_stats(
    project_root: &PathBuf,
    build_type: &str,
    duration: std::time::Duration,
) -> OptimizerResult<()> {
    println!();
    print_status("📊 Build Statistics:");

    // Show basic timing
    println!("  ⏱️  Duration: {}", format_duration(duration));
    println!("  🔧 Build type: {}", build_type);

    // Show target directory size
    let target_dir = project_root.join("target");
    if target_dir.exists() {
        match get_directory_size(&target_dir) {
            Ok(size) => println!("  📁 Target directory size: {}", format_bytes(size)),
            Err(_) => println!("  📁 Target directory size: Unable to calculate"),
        }
    }

    // Show sccache stats if available
    if is_tool_available("sccache") {
        print_status("sccache statistics:");
        let _ = execute_command_with_output("sccache", &["--show-stats"], None);
    }

    // Show dependency count
    if let Ok(output) = execute_command(
        "cargo",
        &["tree", "--workspace", "--depth", "0"],
        Some(project_root),
    ) {
        if output.status.success() {
            if let Ok(stdout) = String::from_utf8(output.stdout) {
                let dep_count = stdout.lines().count();
                println!("  📦 Workspace dependencies: {}", dep_count);
            }
        }
    }

    println!();
    Ok(())
}

use clap::{Parser, Subcommand};
use colored::*;
use std::path::PathBuf;

mod commands;
mod config;
mod error;
mod system;
mod utils;

use commands::*;
use error::OptimizerResult;

#[derive(Parser)]
#[command(
    name = "atlas",
    about = "🚀 A comprehensive Rust build optimization tool",
    long_about = "A comprehensive Rust build optimization tool that dramatically improves build times and development workflow through intelligent configuration, tool management, and performance monitoring.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    /// Suppress all output except errors
    #[arg(short, long, global = true)]
    quiet: bool,

    /// Project directory (defaults to current directory)
    #[arg(short, long, global = true)]
    project_dir: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize optimization for a Rust project
    #[command(alias = "init")]
    Initialize {
        /// Skip backup of existing files
        #[arg(long)]
        no_backup: bool,

        /// Skip installing tools
        #[arg(long)]
        no_tools: bool,

        /// Force overwrite existing configurations
        #[arg(long)]
        force: bool,
    },

    /// Install required optimization tools
    #[command(alias = "tools")]
    InstallTools {
        /// List available tools without installing
        #[arg(long)]
        list: bool,

        /// Install specific tools only
        #[arg(long, value_delimiter = ',')]
        only: Option<Vec<String>>,
    },

    /// Run optimized build commands
    #[command(alias = "build")]
    Build {
        #[command(subcommand)]
        build_type: BuildCommands,
    },

    /// Development workflow commands
    #[command(alias = "dev")]
    Development {
        #[command(subcommand)]
        dev_command: DevCommands,
    },

    /// Analyze and optimize workspace
    #[command(alias = "optimize")]
    Optimize {
        /// Run all optimizations
        #[arg(long)]
        all: bool,

        /// Clean target directory
        #[arg(long)]
        clean: bool,

        /// Check for unused dependencies
        #[arg(long)]
        deps: bool,

        /// Benchmark performance
        #[arg(long)]
        benchmark: bool,
    },

    /// Show optimization status and statistics
    #[command(alias = "status")]
    Status {
        /// Show detailed information
        #[arg(long)]
        detailed: bool,

        /// Export status to JSON
        #[arg(long)]
        json: bool,
    },

    /// Configuration management
    #[command(alias = "config")]
    Config {
        #[command(subcommand)]
        config_command: ConfigCommands,
    },

    /// Update Atlas to the latest version
    Update {
        /// Check for updates without installing
        #[arg(long)]
        check: bool,
    },
}

#[derive(Subcommand)]
enum BuildCommands {
    /// Fast cargo check
    Check {
        /// Show build statistics after completion
        #[arg(long)]
        stats: bool,
    },

    /// Optimized cargo build
    Build {
        /// Build in release mode
        #[arg(long)]
        release: bool,

        /// Show build statistics after completion
        #[arg(long)]
        stats: bool,
    },

    /// Fast testing with cargo-nextest
    Test {
        /// Run only tests for changed files
        #[arg(long)]
        changed: bool,

        /// Show test statistics
        #[arg(long)]
        stats: bool,
    },

    /// Clean build artifacts
    Clean {
        /// Clean everything including rust-analyzer cache
        #[arg(long)]
        all: bool,
    },
}

#[derive(Subcommand)]
enum DevCommands {
    /// Ultra-fast syntax check
    QuickCheck,

    /// Continuous development with auto-rebuild
    Watch {
        /// Watch specific files or directories
        #[arg(long, value_delimiter = ',')]
        paths: Option<Vec<PathBuf>>,
    },

    /// Profile build performance
    Profile {
        /// Generate detailed timing report
        #[arg(long)]
        detailed: bool,
    },

    /// Clean build with maximum optimization
    CleanBuild {
        /// Build in release mode
        #[arg(long)]
        release: bool,
    },
}

#[derive(Subcommand)]
enum ConfigCommands {
    /// Show current configuration
    Show,

    /// Edit configuration file
    Edit,

    /// Reset configuration to defaults
    Reset {
        /// Reset without confirmation
        #[arg(long)]
        force: bool,
    },

    /// Validate current configuration
    Validate,

    /// Export configuration template
    Export {
        /// Output file path
        #[arg(short, long)]
        output: Option<PathBuf>,
    },
}

#[tokio::main]
async fn main() -> OptimizerResult<()> {
    let cli = Cli::parse();

    // Initialize logging
    if !cli.quiet {
        env_logger::Builder::from_default_env()
            .filter_level(if cli.verbose {
                log::LevelFilter::Debug
            } else {
                log::LevelFilter::Info
            })
            .init();
    }

    // Print banner unless quiet mode
    if !cli.quiet {
        print_banner();
    }

    // Execute command
    match cli.command {
        Commands::Initialize {
            no_backup,
            no_tools,
            force,
        } => initialize::run(cli.project_dir, no_backup, no_tools, force).await,
        Commands::InstallTools { list, only } => tools::run(list, only).await,
        Commands::Build { build_type } => build::run(build_type, cli.project_dir).await,
        Commands::Development { dev_command } => {
            development::run(dev_command, cli.project_dir).await
        }
        Commands::Optimize {
            all,
            clean,
            deps,
            benchmark,
        } => optimize::run(all, clean, deps, benchmark, cli.project_dir).await,
        Commands::Status { detailed, json } => status::run(detailed, json, cli.project_dir).await,
        Commands::Config { config_command } => {
            commands::config::run(config_command, cli.project_dir).await
        }
        Commands::Update { check } => update::run(check).await,
    }
}

fn print_banner() {
    println!("{}", "🚀 Atlas".bright_blue().bold());
    println!(
        "{}",
        "Dramatically improve your Rust build times and development workflow".bright_black()
    );
    println!();
}

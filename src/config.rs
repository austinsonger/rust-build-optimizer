use crate::error::{OptimizerError, OptimizerResult};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizerConfig {
    pub build: BuildConfig,
    pub tools: ToolsConfig,
    pub optimization: OptimizationConfig,
    pub development: DevelopmentConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BuildConfig {
    pub parallel_jobs: Option<usize>,
    pub incremental: bool,
    pub target_cpu: String,
    pub use_fast_linker: bool,
    pub separate_rust_analyzer_target: bool,
    pub enable_sccache: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolsConfig {
    pub auto_install: bool,
    pub preferred_tools: Vec<String>,
    pub install_timeout_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationConfig {
    pub clean_old_artifacts: bool,
    pub artifact_retention_days: u32,
    pub check_unused_deps: bool,
    pub optimize_profiles: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DevelopmentConfig {
    pub watch_mode_enabled: bool,
    pub watch_paths: Vec<PathBuf>,
    pub auto_test_on_change: bool,
    pub quick_check_on_save: bool,
}

impl Default for OptimizerConfig {
    fn default() -> Self {
        Self {
            build: BuildConfig {
                parallel_jobs: None, // Auto-detect
                incremental: true,
                target_cpu: "native".to_string(),
                use_fast_linker: true,
                separate_rust_analyzer_target: true,
                enable_sccache: true,
            },
            tools: ToolsConfig {
                auto_install: true,
                preferred_tools: vec![
                    "sccache".to_string(),
                    "cargo-nextest".to_string(),
                    "cargo-udeps".to_string(),
                    "cargo-hakari".to_string(),
                    "cargo-watch".to_string(),
                ],
                install_timeout_seconds: 300,
            },
            optimization: OptimizationConfig {
                clean_old_artifacts: true,
                artifact_retention_days: 7,
                check_unused_deps: true,
                optimize_profiles: true,
            },
            development: DevelopmentConfig {
                watch_mode_enabled: true,
                watch_paths: vec![PathBuf::from("src"), PathBuf::from("Cargo.toml")],
                auto_test_on_change: false,
                quick_check_on_save: true,
            },
        }
    }
}

impl OptimizerConfig {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> OptimizerResult<Self> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|_| OptimizerError::file_not_found(path.as_ref().display().to_string()))?;

        toml::from_str(&content).map_err(OptimizerError::from)
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> OptimizerResult<()> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_config_path() -> OptimizerResult<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| OptimizerError::config("Could not determine config directory"))?;

        Ok(config_dir.join("rust-build-optimizer").join("config.toml"))
    }

    pub fn load_or_default() -> OptimizerResult<Self> {
        let config_path = Self::get_config_path()?;

        if config_path.exists() {
            Self::load_from_file(config_path)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save_default() -> OptimizerResult<()> {
        let config = Self::default();
        let config_path = Self::get_config_path()?;

        // Create config directory if it doesn't exist
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        config.save_to_file(config_path)
    }

    pub fn validate(&self) -> OptimizerResult<()> {
        // Validate parallel jobs
        if let Some(jobs) = self.build.parallel_jobs {
            if jobs == 0 {
                return Err(OptimizerError::config("Parallel jobs cannot be zero"));
            }
            if jobs > 64 {
                return Err(OptimizerError::config("Parallel jobs cannot exceed 64"));
            }
        }

        // Validate artifact retention
        if self.optimization.artifact_retention_days > 365 {
            return Err(OptimizerError::config(
                "Artifact retention cannot exceed 365 days",
            ));
        }

        // Validate tool install timeout
        if self.tools.install_timeout_seconds < 30 {
            return Err(OptimizerError::config(
                "Tool install timeout must be at least 30 seconds",
            ));
        }

        // Validate watch paths exist (if specified)
        for path in &self.development.watch_paths {
            if !path.exists() {
                log::warn!("Watch path does not exist: {}", path.display());
            }
        }

        Ok(())
    }

    pub fn get_effective_parallel_jobs(&self) -> usize {
        self.build.parallel_jobs.unwrap_or_else(num_cpus::get)
    }
}

/// Generate Cargo configuration content
pub fn generate_cargo_config(
    config: &OptimizerConfig,
    system_info: &crate::system::SystemInfo,
) -> String {
    let mut content = String::new();

    content.push_str("# Cargo Configuration for Optimized Builds\n");
    content.push_str("# Generated by rust-build-optimizer\n\n");

    // Build section
    content.push_str("[build]\n");
    content.push_str(&format!(
        "jobs = {}\n",
        config.get_effective_parallel_jobs()
    ));
    content.push_str("target-dir = \"target\"\n");
    content.push_str("pipelining = true\n\n");

    // Environment section
    content.push_str("[env]\n");
    if config.build.separate_rust_analyzer_target {
        content.push_str("CARGO_TARGET_DIR = { value = \"target/rust-analyzer\", condition = \"cfg(rust_analyzer)\" }\n");
    }
    content.push_str(&format!(
        "CARGO_INCREMENTAL = \"{}\"\n",
        if config.build.incremental { "1" } else { "0" }
    ));
    content.push_str("CARGO_PROFILE_DEV_INCREMENTAL = \"true\"\n");
    content.push_str("CARGO_BUILD_CACHE = \"1\"\n");
    content.push_str("CARGO_NET_RETRY = \"3\"\n");
    content.push_str("CARGO_NET_GIT_FETCH_WITH_CLI = \"true\"\n\n");

    // Target-specific configuration
    if config.build.use_fast_linker {
        if let Some(_linker) = system_info.get_recommended_linker() {
            match (&system_info.os, &system_info.arch) {
                (crate::system::OperatingSystem::MacOS, crate::system::Architecture::Aarch64) => {
                    content.push_str("[target.aarch64-apple-darwin]\n");
                    content.push_str("rustflags = [\n");
                    content.push_str(&format!(
                        "    \"-C\", \"target-cpu={}\",\n",
                        config.build.target_cpu
                    ));
                    content.push_str("    \"-C\", \"codegen-units=16\",\n");
                    content.push_str("    \"-C\", \"link-arg=-Wl,-dead_strip\",\n");
                    content.push_str("    \"-C\", \"link-arg=-Wl,-no_compact_unwind\",\n");
                    content.push_str("]\n\n");
                }
                (crate::system::OperatingSystem::MacOS, _) => {
                    content.push_str("[target.x86_64-apple-darwin]\n");
                    content.push_str("linker = \"clang\"\n");
                    content.push_str("rustflags = [\n");
                    content.push_str("    \"-C\", \"link-arg=-fuse-ld=/usr/local/bin/zld\",\n");
                    content.push_str(&format!(
                        "    \"-C\", \"target-cpu={}\",\n",
                        config.build.target_cpu
                    ));
                    content.push_str("    \"-C\", \"codegen-units=1\",\n");
                    content.push_str("]\n\n");
                }
                (crate::system::OperatingSystem::Linux, _) => {
                    content.push_str("[target.x86_64-unknown-linux-gnu]\n");
                    content.push_str("linker = \"clang\"\n");
                    content.push_str("rustflags = [\n");
                    content.push_str("    \"-C\", \"link-arg=-fuse-ld=mold\",\n");
                    content.push_str(&format!(
                        "    \"-C\", \"target-cpu={}\",\n",
                        config.build.target_cpu
                    ));
                    content.push_str("    \"-C\", \"codegen-units=1\",\n");
                    content.push_str("]\n\n");
                }
                _ => {}
            }
        }
    }

    // Registry configuration
    content.push_str("[registries.crates-io]\n");
    content.push_str("protocol = \"sparse\"\n\n");

    // Network configuration
    content.push_str("[net]\n");
    content.push_str("retry = 3\n");
    content.push_str("git-fetch-with-cli = true\n");

    content
}

/// Generate optimized Cargo.toml profiles
pub fn generate_cargo_profiles() -> String {
    r#"# Optimized build profiles for better performance and faster compilation
[profile.dev]
# Enable incremental compilation for faster rebuilds
incremental = true
# Optimize for compilation speed in development
opt-level = 0
# Enable debug info for better debugging experience
debug = true
# Reduce binary size in development
strip = false
# Use more codegen units for faster parallel compilation
codegen-units = 512
# Enable overflow checks in development
overflow-checks = true
# Enable debug assertions
debug-assertions = true
# Faster compilation with less optimization
lto = false
# Enable panic unwinding for better error messages
panic = "unwind"

[profile.dev.package."*"]
# Optimize dependencies even in dev mode for better performance
opt-level = 3
# Disable debug info for dependencies to speed up compilation
debug = false

[profile.release]
# Maximum optimization for production builds
opt-level = 3
# Disable debug info in release builds
debug = false
# Strip symbols to reduce binary size
strip = "symbols"
# Use single codegen unit for better optimization
codegen-units = 1
# Enable Link Time Optimization for better performance
lto = "thin"
# Enable overflow checks even in release (security)
overflow-checks = true
# Disable debug assertions in release
debug-assertions = false
# Abort on panic for smaller binary size
panic = "abort"

[profile.release-with-debug]
# Release profile with debug info for profiling
inherits = "release"
debug = true
strip = false

[profile.bench]
# Optimized profile for benchmarking
inherits = "release"
debug = true
lto = true

[profile.test]
# Optimized profile for testing
inherits = "dev"
opt-level = 1
# Faster test compilation
codegen-units = 512
"#
    .to_string()
}

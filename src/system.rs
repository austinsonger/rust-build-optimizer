use crate::error::OptimizerResult;
use serde::{Deserialize, Serialize};
use std::process::Command;
use which::which;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub os: OperatingSystem,
    pub arch: Architecture,
    pub cpu_cores: usize,
    pub rust_version: Option<String>,
    pub cargo_version: Option<String>,
    pub available_tools: Vec<AvailableTool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum OperatingSystem {
    MacOS,
    Linux,
    Windows,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Architecture {
    X86_64,
    Aarch64,
    Unknown(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AvailableTool {
    pub name: String,
    pub version: Option<String>,
    pub path: String,
    pub is_installed: bool,
}

impl SystemInfo {
    pub fn detect() -> OptimizerResult<Self> {
        let os = detect_os();
        let arch = detect_architecture();
        let cpu_cores = detect_cpu_cores();
        let rust_version = detect_rust_version();
        let cargo_version = detect_cargo_version();
        let available_tools = detect_available_tools();

        Ok(SystemInfo {
            os,
            arch,
            cpu_cores,
            rust_version,
            cargo_version,
            available_tools,
        })
    }

    pub fn get_recommended_linker(&self) -> Option<&'static str> {
        match (&self.os, &self.arch) {
            (OperatingSystem::MacOS, Architecture::Aarch64) => Some("system"),
            (OperatingSystem::MacOS, _) => Some("zld"),
            (OperatingSystem::Linux, _) => Some("mold"),
            (OperatingSystem::Windows, _) => Some("lld"),
            _ => None,
        }
    }

    pub fn get_package_manager(&self) -> Option<&'static str> {
        match self.os {
            OperatingSystem::MacOS => Some("brew"),
            OperatingSystem::Linux => {
                // Try to detect Linux distribution
                if which("apt-get").is_ok() {
                    Some("apt")
                } else if which("yum").is_ok() {
                    Some("yum")
                } else if which("pacman").is_ok() {
                    Some("pacman")
                } else {
                    None
                }
            }
            OperatingSystem::Windows => Some("winget"),
            _ => None,
        }
    }

    pub fn supports_fast_linker(&self) -> bool {
        self.get_recommended_linker().is_some()
    }

    pub fn get_tool(&self, name: &str) -> Option<&AvailableTool> {
        self.available_tools.iter().find(|tool| tool.name == name)
    }

    pub fn is_tool_installed(&self, name: &str) -> bool {
        self.get_tool(name)
            .map(|tool| tool.is_installed)
            .unwrap_or(false)
    }
}

fn detect_os() -> OperatingSystem {
    match std::env::consts::OS {
        "macos" => OperatingSystem::MacOS,
        "linux" => OperatingSystem::Linux,
        "windows" => OperatingSystem::Windows,
        other => OperatingSystem::Unknown(other.to_string()),
    }
}

fn detect_architecture() -> Architecture {
    match std::env::consts::ARCH {
        "x86_64" => Architecture::X86_64,
        "aarch64" => Architecture::Aarch64,
        other => Architecture::Unknown(other.to_string()),
    }
}

fn detect_cpu_cores() -> usize {
    num_cpus::get()
}

fn detect_rust_version() -> Option<String> {
    Command::new("rustc")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

fn detect_cargo_version() -> Option<String> {
    Command::new("cargo")
        .arg("--version")
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
}

fn detect_available_tools() -> Vec<AvailableTool> {
    let tools = vec![
        "sccache",
        "cargo-nextest",
        "cargo-udeps",
        "cargo-hakari",
        "cargo-watch",
        "cargo-expand",
        "cargo-bloat",
        "lld",
        "mold",
        "zld",
        "clang",
        "gcc",
    ];

    tools
        .into_iter()
        .map(|name| {
            let (is_installed, path, version) = if let Ok(tool_path) = which(name) {
                let version = get_tool_version(name);
                (true, tool_path.to_string_lossy().to_string(), version)
            } else {
                (false, String::new(), None)
            };

            AvailableTool {
                name: name.to_string(),
                version,
                path,
                is_installed,
            }
        })
        .collect()
}

fn get_tool_version(tool: &str) -> Option<String> {
    let version_args = match tool {
        "sccache" => vec!["--version"],
        "cargo-nextest" | "cargo-udeps" | "cargo-hakari" | "cargo-watch" | "cargo-expand"
        | "cargo-bloat" => {
            vec!["--version"]
        }
        "lld" => vec!["--version"],
        "mold" => vec!["--version"],
        "zld" => vec!["--version"],
        "clang" | "gcc" => vec!["--version"],
        _ => return None,
    };

    Command::new(tool)
        .args(&version_args)
        .output()
        .ok()
        .and_then(|output| {
            if output.status.success() {
                String::from_utf8(output.stdout).ok().and_then(|s| {
                    // Extract version number from output
                    s.lines().next().map(|line| line.trim().to_string())
                })
            } else {
                None
            }
        })
}

impl std::fmt::Display for OperatingSystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MacOS => write!(f, "macOS"),
            Self::Linux => write!(f, "Linux"),
            Self::Windows => write!(f, "Windows"),
            Self::Unknown(s) => write!(f, "Unknown ({})", s),
        }
    }
}

impl std::fmt::Display for Architecture {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::X86_64 => write!(f, "x86_64"),
            Self::Aarch64 => write!(f, "aarch64"),
            Self::Unknown(s) => write!(f, "Unknown ({})", s),
        }
    }
}

// Add num_cpus dependency to Cargo.toml
extern crate num_cpus;

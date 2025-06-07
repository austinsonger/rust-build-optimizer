use crate::error::{OptimizerError, OptimizerResult};
use crate::system::SystemInfo;
use crate::utils::*;
use colored::*;
use indicatif::ProgressBar;
use std::collections::HashMap;
use std::process::Command;
use std::time::Duration;

pub async fn run(list: bool, only: Option<Vec<String>>) -> OptimizerResult<()> {
    let system_info = SystemInfo::detect()?;
    
    if list {
        list_available_tools(&system_info).await?;
        return Ok(());
    }

    let tools_to_install = if let Some(specific_tools) = only {
        specific_tools
    } else {
        get_recommended_tools(&system_info)
    };

    install_tools(&tools_to_install).await?;
    
    Ok(())
}

pub async fn install_tools(tools: &[String]) -> OptimizerResult<()> {
    print_status("Installing optimization tools...");
    
    let system_info = SystemInfo::detect()?;
    let mut results = HashMap::new();
    
    for tool in tools {
        print_status(&format!("Installing {}...", tool.bright_cyan()));
        
        let result = install_single_tool(tool, &system_info).await;
        results.insert(tool.clone(), result);
        
        match &results[tool] {
            Ok(_) => print_success(&format!("✅ {} installed successfully", tool)),
            Err(e) => print_warning(&format!("⚠️  Failed to install {}: {}", tool, e)),
        }
    }
    
    // Print summary
    print_installation_summary(&results);
    
    Ok(())
}

async fn install_single_tool(tool: &str, system_info: &SystemInfo) -> OptimizerResult<()> {
    // Check if tool is already installed
    if system_info.is_tool_installed(tool) {
        return Ok(());
    }
    
    let spinner = create_spinner(&format!("Installing {}", tool));
    
    let result = match tool {
        "sccache" => install_sccache(system_info).await,
        "cargo-nextest" => install_cargo_tool("cargo-nextest").await,
        "cargo-udeps" => install_cargo_tool("cargo-udeps").await,
        "cargo-hakari" => install_cargo_tool("cargo-hakari").await,
        "cargo-watch" => install_cargo_tool("cargo-watch").await,
        "cargo-expand" => install_cargo_tool("cargo-expand").await,
        "cargo-bloat" => install_cargo_tool("cargo-bloat").await,
        "mold" => install_mold(system_info).await,
        "zld" => install_zld(system_info).await,
        "lld" => install_lld(system_info).await,
        _ => Err(OptimizerError::tool_not_found(format!("Unknown tool: {}", tool))),
    };
    
    spinner.finish_and_clear();
    result
}

async fn install_sccache(system_info: &SystemInfo) -> OptimizerResult<()> {
    match &system_info.os {
        crate::system::OperatingSystem::MacOS => {
            execute_command_with_output("brew", &["install", "sccache"], None)
        }
        crate::system::OperatingSystem::Linux => {
            // Try package manager first, fall back to cargo
            if let Some(pm) = system_info.get_package_manager() {
                match pm {
                    "apt" => {
                        if execute_command_success("sudo", &["apt-get", "update"], None)? {
                            execute_command_with_output("sudo", &["apt-get", "install", "-y", "sccache"], None)
                        } else {
                            install_cargo_tool("sccache").await
                        }
                    }
                    "yum" => {
                        execute_command_with_output("sudo", &["yum", "install", "-y", "sccache"], None)
                            .or_else(|_| install_cargo_tool("sccache"))
                    }
                    "pacman" => {
                        execute_command_with_output("sudo", &["pacman", "-S", "--noconfirm", "sccache"], None)
                            .or_else(|_| install_cargo_tool("sccache"))
                    }
                    _ => install_cargo_tool("sccache").await,
                }
            } else {
                install_cargo_tool("sccache").await
            }
        }
        crate::system::OperatingSystem::Windows => {
            execute_command_with_output("winget", &["install", "Mozilla.sccache"], None)
                .or_else(|_| install_cargo_tool("sccache"))
        }
        _ => install_cargo_tool("sccache").await,
    }
}

async fn install_cargo_tool(tool: &str) -> OptimizerResult<()> {
    execute_command_with_output("cargo", &["install", tool, "--locked"], None)
}

async fn install_mold(system_info: &SystemInfo) -> OptimizerResult<()> {
    match &system_info.os {
        crate::system::OperatingSystem::Linux => {
            if let Some(pm) = system_info.get_package_manager() {
                match pm {
                    "apt" => execute_command_with_output("sudo", &["apt-get", "install", "-y", "mold"], None),
                    "yum" => execute_command_with_output("sudo", &["yum", "install", "-y", "mold"], None),
                    "pacman" => execute_command_with_output("sudo", &["pacman", "-S", "--noconfirm", "mold"], None),
                    _ => Err(OptimizerError::unsupported_platform("Package manager not supported for mold installation")),
                }
            } else {
                Err(OptimizerError::unsupported_platform("No package manager found for mold installation"))
            }
        }
        _ => Err(OptimizerError::unsupported_platform("mold is only available on Linux")),
    }
}

async fn install_zld(system_info: &SystemInfo) -> OptimizerResult<()> {
    match &system_info.os {
        crate::system::OperatingSystem::MacOS => {
            execute_command_with_output("brew", &["install", "zld"], None)
        }
        _ => Err(OptimizerError::unsupported_platform("zld is only available on macOS")),
    }
}

async fn install_lld(system_info: &SystemInfo) -> OptimizerResult<()> {
    match &system_info.os {
        crate::system::OperatingSystem::MacOS => {
            execute_command_with_output("brew", &["install", "llvm"], None)
        }
        crate::system::OperatingSystem::Linux => {
            if let Some(pm) = system_info.get_package_manager() {
                match pm {
                    "apt" => execute_command_with_output("sudo", &["apt-get", "install", "-y", "lld"], None),
                    "yum" => execute_command_with_output("sudo", &["yum", "install", "-y", "lld"], None),
                    "pacman" => execute_command_with_output("sudo", &["pacman", "-S", "--noconfirm", "lld"], None),
                    _ => Err(OptimizerError::unsupported_platform("Package manager not supported for lld installation")),
                }
            } else {
                Err(OptimizerError::unsupported_platform("No package manager found for lld installation"))
            }
        }
        crate::system::OperatingSystem::Windows => {
            // LLD comes with LLVM on Windows
            execute_command_with_output("winget", &["install", "LLVM.LLVM"], None)
        }
        _ => Err(OptimizerError::unsupported_platform("lld installation not supported on this platform")),
    }
}

async fn list_available_tools(system_info: &SystemInfo) -> OptimizerResult<()> {
    println!("{}", "📦 Available Optimization Tools".bright_blue().bold());
    println!();

    let tools = get_all_tools();
    
    for (category, tool_list) in tools {
        println!("{}", category.bright_green().bold());
        
        for tool in tool_list {
            let status = if system_info.is_tool_installed(&tool.name) {
                "✅ Installed".bright_green()
            } else {
                "❌ Not installed".bright_red()
            };
            
            let platform_support = if is_tool_supported(&tool.name, system_info) {
                "✅ Supported".bright_green()
            } else {
                "❌ Not supported".bright_red()
            };
            
            println!("  {} - {} | {} | {}", 
                tool.name.bright_cyan(),
                tool.description,
                status,
                platform_support
            );
        }
        println!();
    }

    Ok(())
}

fn get_recommended_tools(system_info: &SystemInfo) -> Vec<String> {
    let mut tools = vec![
        "sccache".to_string(),
        "cargo-nextest".to_string(),
        "cargo-udeps".to_string(),
        "cargo-hakari".to_string(),
        "cargo-watch".to_string(),
    ];

    // Add platform-specific linkers
    match &system_info.os {
        crate::system::OperatingSystem::MacOS => {
            if system_info.arch == crate::system::Architecture::X86_64 {
                tools.push("zld".to_string());
            }
        }
        crate::system::OperatingSystem::Linux => {
            tools.push("mold".to_string());
        }
        crate::system::OperatingSystem::Windows => {
            tools.push("lld".to_string());
        }
        _ => {}
    }

    tools
}

struct Tool {
    name: String,
    description: String,
}

fn get_all_tools() -> Vec<(&'static str, Vec<Tool>)> {
    vec![
        ("🚀 Build Acceleration", vec![
            Tool { name: "sccache".to_string(), description: "Compilation cache for faster builds".to_string() },
            Tool { name: "cargo-nextest".to_string(), description: "Fast test runner".to_string() },
        ]),
        ("🔗 Fast Linkers", vec![
            Tool { name: "mold".to_string(), description: "Fastest linker for Linux".to_string() },
            Tool { name: "zld".to_string(), description: "Fast linker for macOS".to_string() },
            Tool { name: "lld".to_string(), description: "LLVM linker (cross-platform)".to_string() },
        ]),
        ("🔍 Analysis Tools", vec![
            Tool { name: "cargo-udeps".to_string(), description: "Find unused dependencies".to_string() },
            Tool { name: "cargo-hakari".to_string(), description: "Workspace optimization".to_string() },
            Tool { name: "cargo-expand".to_string(), description: "Macro expansion".to_string() },
            Tool { name: "cargo-bloat".to_string(), description: "Binary size analysis".to_string() },
        ]),
        ("⚡ Development Tools", vec![
            Tool { name: "cargo-watch".to_string(), description: "Auto-rebuild on file changes".to_string() },
        ]),
    ]
}

fn is_tool_supported(tool: &str, system_info: &SystemInfo) -> bool {
    match tool {
        "mold" => matches!(system_info.os, crate::system::OperatingSystem::Linux),
        "zld" => matches!(system_info.os, crate::system::OperatingSystem::MacOS),
        _ => true, // Most tools are cross-platform
    }
}

fn print_installation_summary(results: &HashMap<String, OptimizerResult<()>>) {
    println!();
    println!("{}", "📊 Installation Summary".bright_blue().bold());
    println!();
    
    let successful: Vec<_> = results.iter().filter(|(_, result)| result.is_ok()).collect();
    let failed: Vec<_> = results.iter().filter(|(_, result)| result.is_err()).collect();
    
    if !successful.is_empty() {
        println!("{} Successfully installed:", "✅".bright_green());
        for (tool, _) in successful {
            println!("  • {}", tool.bright_green());
        }
        println!();
    }
    
    if !failed.is_empty() {
        println!("{} Failed to install:", "❌".bright_red());
        for (tool, error) in failed {
            println!("  • {}: {}", tool.bright_red(), error.as_ref().unwrap_err());
        }
        println!();
        println!("{} You can install these tools manually or try again later.", "💡".bright_yellow());
    }
    
    println!("🎉 Tool installation completed!");
}

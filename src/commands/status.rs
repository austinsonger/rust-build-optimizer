use crate::error::OptimizerResult;
use crate::system::SystemInfo;
use crate::utils::*;
use colored::*;
use std::path::PathBuf;

pub async fn run(detailed: bool, json: bool, project_dir: Option<PathBuf>) -> OptimizerResult<()> {
    let _project_root = if let Some(dir) = project_dir {
        dir
    } else {
        find_rust_project_root(".")?
    };

    let system_info = SystemInfo::detect()?;

    if json {
        let status = serde_json::json!({
            "system": {
                "os": system_info.os.to_string(),
                "arch": system_info.arch.to_string(),
                "cpu_cores": system_info.cpu_cores,
                "rust_version": system_info.rust_version,
                "cargo_version": system_info.cargo_version
            },
            "tools": system_info.available_tools
        });
        println!("{}", serde_json::to_string_pretty(&status)?);
    } else {
        print_status_overview(&system_info, detailed);
    }

    Ok(())
}

fn print_status_overview(system_info: &SystemInfo, detailed: bool) {
    println!("{}", "🚀 Rust Build Optimizer Status".bright_blue().bold());
    println!();

    // System information
    println!("{}", "💻 System Information".bright_green().bold());
    println!("  OS: {} {}", system_info.os, system_info.arch);
    println!("  CPU Cores: {}", system_info.cpu_cores);
    if let Some(ref rust_version) = system_info.rust_version {
        println!("  Rust: {}", rust_version);
    }
    if let Some(ref cargo_version) = system_info.cargo_version {
        println!("  Cargo: {}", cargo_version);
    }
    println!();

    // Tool status
    println!("{}", "🛠️  Tool Status".bright_green().bold());
    for tool in &system_info.available_tools {
        let status = if tool.is_installed {
            "✅ Installed".bright_green()
        } else {
            "❌ Not installed".bright_red()
        };
        
        if detailed && tool.is_installed {
            if let Some(ref version) = tool.version {
                println!("  {} - {} ({})", tool.name.bright_cyan(), status, version);
            } else {
                println!("  {} - {}", tool.name.bright_cyan(), status);
            }
        } else {
            println!("  {} - {}", tool.name.bright_cyan(), status);
        }
    }
    println!();

    // Recommendations
    let missing_tools: Vec<_> = system_info.available_tools
        .iter()
        .filter(|tool| !tool.is_installed)
        .collect();

    if !missing_tools.is_empty() {
        println!("{}", "💡 Recommendations".bright_yellow().bold());
        println!("  Install missing tools with: {}", 
            "rust-build-optimizer install-tools".bright_cyan());
        for tool in missing_tools {
            println!("    • {}", tool.name);
        }
        println!();
    }

    println!("🎉 Status check completed!");
}

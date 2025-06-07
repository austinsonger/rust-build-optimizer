use crate::error::OptimizerResult;
use crate::utils::*;
use crate::DevCommands;
use std::path::PathBuf;

pub async fn run(dev_command: DevCommands, project_dir: Option<PathBuf>) -> OptimizerResult<()> {
    let _project_root = if let Some(dir) = project_dir {
        dir
    } else {
        find_rust_project_root(".")?
    };

    match dev_command {
        DevCommands::QuickCheck => {
            print_status("Running ultra-fast syntax check...");
            execute_command_with_output("cargo", &["check", "--lib", "--bins", "--workspace", "--message-format=short"], None)?;
            print_success("✅ Quick check completed");
            Ok(())
        }
        DevCommands::Watch { paths: _ } => {
            print_status("Starting watch mode...");
            if is_tool_available("cargo-watch") {
                execute_command_with_output("cargo", &["watch", "-x", "check --workspace --message-format=short"], None)?;
            } else {
                print_warning("cargo-watch not installed. Install with: cargo install cargo-watch");
            }
            Ok(())
        }
        DevCommands::Profile { detailed: _ } => {
            print_status("Profiling build performance...");
            execute_command_with_output("cargo", &["build", "--timings"], None)?;
            print_success("✅ Build profile generated (see cargo-timing.html)");
            Ok(())
        }
        DevCommands::CleanBuild { release } => {
            print_status("Running clean build...");
            execute_command_with_output("cargo", &["clean"], None)?;
            if release {
                execute_command_with_output("cargo", &["build", "--release"], None)?;
            } else {
                execute_command_with_output("cargo", &["build"], None)?;
            }
            print_success("✅ Clean build completed");
            Ok(())
        }
    }
}

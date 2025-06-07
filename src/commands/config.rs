use crate::config::OptimizerConfig;
use crate::error::OptimizerResult;
use crate::utils::*;
use crate::ConfigCommands;
use std::path::PathBuf;

pub async fn run(config_command: ConfigCommands, _project_dir: Option<PathBuf>) -> OptimizerResult<()> {
    match config_command {
        ConfigCommands::Show => {
            let config = OptimizerConfig::load_or_default()?;
            println!("{}", toml::to_string_pretty(&config)?);
            Ok(())
        }
        ConfigCommands::Edit => {
            let config_path = OptimizerConfig::get_config_path()?;
            print_status(&format!("Edit configuration file: {}", config_path.display()));
            
            // Try to open with default editor
            if let Ok(editor) = std::env::var("EDITOR") {
                execute_command_with_output(&editor, &[config_path.to_str().unwrap()], None)?;
            } else {
                print_warning("No EDITOR environment variable set. Please edit manually:");
                println!("{}", config_path.display());
            }
            Ok(())
        }
        ConfigCommands::Reset { force } => {
            if force || confirm("Reset configuration to defaults?")? {
                OptimizerConfig::save_default()?;
                print_success("✅ Configuration reset to defaults");
            }
            Ok(())
        }
        ConfigCommands::Validate => {
            let config = OptimizerConfig::load_or_default()?;
            config.validate()?;
            print_success("✅ Configuration is valid");
            Ok(())
        }
        ConfigCommands::Export { output } => {
            let config = OptimizerConfig::load_or_default()?;
            let content = toml::to_string_pretty(&config)?;
            
            if let Some(output_path) = output {
                std::fs::write(&output_path, content)?;
                print_success(&format!("✅ Configuration exported to {}", output_path.display()));
            } else {
                println!("{}", content);
            }
            Ok(())
        }
    }
}

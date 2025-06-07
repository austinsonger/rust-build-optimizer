use thiserror::Error;
use std::io;

pub type OptimizerResult<T> = Result<T, OptimizerError>;

#[derive(Error, Debug)]
pub enum OptimizerError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Tool not found: {0}")]
    ToolNotFound(String),

    #[error("Command execution failed: {0}")]
    CommandFailed(String),

    #[error("Project validation failed: {0}")]
    ProjectValidation(String),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("TOML parsing error: {0}")]
    TomlParsing(#[from] toml::de::Error),

    #[error("TOML serialization error: {0}")]
    TomlSerialization(#[from] toml::ser::Error),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Permission denied: {0}")]
    Permission(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Operation cancelled by user")]
    Cancelled,

    #[error("Unsupported platform: {0}")]
    UnsupportedPlatform(String),

    #[error("Tool installation failed: {tool} - {reason}")]
    ToolInstallation { tool: String, reason: String },

    #[error("Build failed: {0}")]
    BuildFailed(String),

    #[error("Test failed: {0}")]
    TestFailed(String),

    #[error("Optimization failed: {0}")]
    OptimizationFailed(String),

    #[error("Multiple errors occurred: {0:?}")]
    Multiple(Vec<OptimizerError>),
}

impl OptimizerError {
    pub fn config<S: Into<String>>(msg: S) -> Self {
        Self::Config(msg.into())
    }

    pub fn tool_not_found<S: Into<String>>(tool: S) -> Self {
        Self::ToolNotFound(tool.into())
    }

    pub fn command_failed<S: Into<String>>(msg: S) -> Self {
        Self::CommandFailed(msg.into())
    }

    pub fn project_validation<S: Into<String>>(msg: S) -> Self {
        Self::ProjectValidation(msg.into())
    }

    pub fn network<S: Into<String>>(msg: S) -> Self {
        Self::Network(msg.into())
    }

    pub fn permission<S: Into<String>>(msg: S) -> Self {
        Self::Permission(msg.into())
    }

    pub fn file_not_found<S: Into<String>>(path: S) -> Self {
        Self::FileNotFound(path.into())
    }

    pub fn invalid_input<S: Into<String>>(msg: S) -> Self {
        Self::InvalidInput(msg.into())
    }

    pub fn unsupported_platform<S: Into<String>>(platform: S) -> Self {
        Self::UnsupportedPlatform(platform.into())
    }

    pub fn tool_installation<S: Into<String>>(tool: S, reason: S) -> Self {
        Self::ToolInstallation {
            tool: tool.into(),
            reason: reason.into(),
        }
    }

    pub fn build_failed<S: Into<String>>(msg: S) -> Self {
        Self::BuildFailed(msg.into())
    }

    pub fn test_failed<S: Into<String>>(msg: S) -> Self {
        Self::TestFailed(msg.into())
    }

    pub fn optimization_failed<S: Into<String>>(msg: S) -> Self {
        Self::OptimizationFailed(msg.into())
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            Self::Io(_) => false,
            Self::Config(_) => true,
            Self::ToolNotFound(_) => true,
            Self::CommandFailed(_) => true,
            Self::ProjectValidation(_) => true,
            Self::Serialization(_) => false,
            Self::TomlParsing(_) => true,
            Self::TomlSerialization(_) => false,
            Self::Network(_) => true,
            Self::Permission(_) => false,
            Self::FileNotFound(_) => true,
            Self::InvalidInput(_) => true,
            Self::Cancelled => true,
            Self::UnsupportedPlatform(_) => false,
            Self::ToolInstallation { .. } => true,
            Self::BuildFailed(_) => true,
            Self::TestFailed(_) => true,
            Self::OptimizationFailed(_) => true,
            Self::Multiple(errors) => errors.iter().any(|e| e.is_recoverable()),
        }
    }

    /// Get user-friendly error message with suggestions
    pub fn user_message(&self) -> String {
        match self {
            Self::ToolNotFound(tool) => {
                format!(
                    "Tool '{}' is not installed. Run 'rust-build-optimizer install-tools' to install it.",
                    tool
                )
            }
            Self::ProjectValidation(msg) => {
                format!(
                    "Project validation failed: {}. Make sure you're in a Rust project directory.",
                    msg
                )
            }
            Self::CommandFailed(msg) => {
                format!(
                    "Command failed: {}. Check your project configuration and try again.",
                    msg
                )
            }
            Self::Permission(msg) => {
                format!(
                    "Permission denied: {}. You may need to run with elevated privileges.",
                    msg
                )
            }
            Self::UnsupportedPlatform(platform) => {
                format!(
                    "Platform '{}' is not supported. Please check the documentation for supported platforms.",
                    platform
                )
            }
            Self::ToolInstallation { tool, reason } => {
                format!(
                    "Failed to install '{}': {}. Please install it manually or check your internet connection.",
                    tool, reason
                )
            }
            _ => self.to_string(),
        }
    }
}

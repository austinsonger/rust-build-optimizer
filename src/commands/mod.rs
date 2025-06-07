pub mod initialize;
pub mod tools;
pub mod build;
pub mod development;
pub mod optimize;
pub mod status;
pub mod config;
pub mod update;

// Re-export command functions
pub use initialize::run as initialize;
pub use tools::run as tools;
pub use build::run as build;
pub use development::run as development;
pub use optimize::run as optimize;
pub use status::run as status;
pub use config::run as config;
pub use update::run as update;

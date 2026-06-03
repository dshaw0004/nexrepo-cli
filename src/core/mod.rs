// Declare submodules (private by default)
mod config;
// mod git; // Future: mod git;
// mod remote; // Future: mod remote;

// Re-export the public API of this module
// This allows users to write: use nexrepo::core::Config;
// Instead of: use nexrepo::core::config::Config;
pub use config::{Config, ProjectConfig};

// You can also re-export functions if needed:
// pub use git::run_git_command;

use dirs::home_dir;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// The root configuration structure stored in ~/.nexrepo/config.toml
#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    // Maps the absolute path of a local git repo to its nexrepo settings
    #[serde(default)]
    pub projects: HashMap<String, ProjectConfig>,
}

/// Settings for a specific local repository
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ProjectConfig {
    /// The "source of truth" remote (e.g., "github")
    pub primary_remote: String,
    /// List of all remotes managed by nexrepo (e.g., ["github", "gitlab"])
    pub remotes: Vec<String>,
}

impl Config {
    /// Gets the path to ~/.nexrepo/config.toml
    fn get_path() -> Result<PathBuf> {
        let mut path = home_dir().context("Could not find your home directory")?;
        path.push(".nexrepo");
        path.push("config.toml");
        Ok(path)
    }

    /// Loads the config from disk, or returns an empty default config
    pub fn load() -> Result<Self> {
        let path = Self::get_path()?;
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(&path).context("Failed to read nexrepo config file")?;
        toml::from_str(&content).context("Failed to parse nexrepo config file")
    }

    /// Saves the current config state to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::get_path()?;

        // Create ~/.nexrepo/ directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;
        fs::write(path, content).context("Failed to write nexrepo config file")
    }

    /// Helper to get the config for the current directory
    pub fn get_current_project(&self, repo_path: &str) -> Option<&ProjectConfig> {
        self.projects.get(repo_path)
    }
}

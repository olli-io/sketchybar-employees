//! Configuration module for sketchybartender update intervals

use std::env;
use std::fs;
use std::path::PathBuf;

/// Configuration for update intervals (in seconds)
#[derive(Debug, Clone)]
pub struct Config {
    /// Clock update interval (default: 15 seconds)
    pub clock_interval: u64,
    /// Battery update interval (default: 120 seconds)
    pub battery_interval: u64,
    /// Brew outdated check interval (default: 3600 seconds / 1 hour)
    pub brew_interval: u64,
    /// Teams notification check interval (default: 30 seconds)
    pub teams_interval: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            clock_interval: 15,
            battery_interval: 120,
            brew_interval: 3600,
            teams_interval: 30,
        }
    }
}

impl Config {
    /// Load configuration from file or use defaults
    pub fn load() -> Self {
        let config_path = Self::get_config_path();

        if config_path.exists() {
            match Self::load_from_file(&config_path) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Failed to load config from {:?}: {}", config_path, e);
                    eprintln!("Using default configuration");
                    Self::default()
                }
            }
        } else {
            // Create default config file
            let config = Self::default();
            if let Err(e) = config.save_to_file(&config_path) {
                eprintln!("Failed to save default config: {}", e);
            } else {
                eprintln!("Created default config at {:?}", config_path);
            }
            config
        }
    }

    /// Get the configuration file path
    fn get_config_path() -> PathBuf {
        let config_dir = env::var("XDG_CONFIG_HOME")
            .map(PathBuf::from)
            .unwrap_or_else(|_| {
                let home = env::var("HOME").expect("HOME not set");
                PathBuf::from(home).join(".config")
            });

        config_dir.join("sketchybar").join("sketchybartenderrc")
    }

    /// Load configuration from a file
    fn load_from_file(path: &PathBuf) -> Result<Self, String> {
        let contents = fs::read_to_string(path)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        let mut config = Self::default();

        for line in contents.lines() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            // Parse key=value pairs
            if let Some((key, value)) = line.split_once('=') {
                let key = key.trim();
                let value = value.trim();

                match key {
                    "clock_interval" => {
                        config.clock_interval = value.parse()
                            .map_err(|_| format!("Invalid value for clock_interval: {}", value))?;
                    }
                    "battery_interval" => {
                        config.battery_interval = value.parse()
                            .map_err(|_| format!("Invalid value for battery_interval: {}", value))?;
                    }
                    "brew_interval" => {
                        config.brew_interval = value.parse()
                            .map_err(|_| format!("Invalid value for brew_interval: {}", value))?;
                    }
                    "teams_interval" => {
                        config.teams_interval = value.parse()
                            .map_err(|_| format!("Invalid value for teams_interval: {}", value))?;
                    }
                    _ => {
                        eprintln!("Warning: Unknown config key: {}", key);
                    }
                }
            }
        }

        Ok(config)
    }

    /// Save configuration to a file
    fn save_to_file(&self, path: &PathBuf) -> Result<(), String> {
        // Ensure parent directory exists
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("Failed to create config directory: {}", e))?;
        }

        let contents = format!(
            "# Sketchybartender Configuration\n\
             # Update intervals in seconds\n\
             \n\
             # Clock update interval (default: 15)\n\
             clock_interval = {}\n\
             \n\
             # Battery update interval (default: 120)\n\
             battery_interval = {}\n\
             \n\
             # Brew outdated check interval (default: 3600)\n\
             brew_interval = {}\n\
             \n\
             # Teams notification check interval (default: 30)\n\
             teams_interval = {}\n",
            self.clock_interval,
            self.battery_interval,
            self.brew_interval,
            self.teams_interval,
        );

        fs::write(path, contents)
            .map_err(|e| format!("Failed to write config file: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.clock_interval, 15);
        assert_eq!(config.battery_interval, 120);
        assert_eq!(config.brew_interval, 3600);
        assert_eq!(config.teams_interval, 30);
    }
}

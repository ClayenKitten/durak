mod default_name;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use std::path::PathBuf;

use self::default_name::generate_default_name;

/// Configuration file.
#[derive(Debug, Resource, Serialize, Deserialize)]
pub struct Configuration {
    pub name: String,
    pub server_address: String,
}

impl Configuration {
    /// Loads configuration from the disk.
    pub fn load() -> Result<Self, ConfigurationError> {
        let value = std::fs::read_to_string(Self::path()?).map_err(|_| ConfigurationError)?;
        let parsed = toml::from_str::<Configuration>(&value).map_err(|_| ConfigurationError)?;
        Ok(parsed)
    }

    /// Saves current configuration to disk.
    pub fn save(&self) -> Result<(), ConfigurationError> {
        let value = toml::to_string_pretty(self).map_err(|_| ConfigurationError)?;
        std::fs::write(Self::path()?, value).map_err(|_| ConfigurationError)?;
        Ok(())
    }

    /// Returns path to the configuration file.
    pub fn path() -> Result<PathBuf, ConfigurationError> {
        let mut path = std::env::current_exe().map_err(|_| ConfigurationError)?;
        path.pop();
        path.push(PathBuf::from("data.toml"));
        Ok(path)
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
            name: generate_default_name(),
            server_address: String::from(env!("DURAK_SERVER_ADDRESS")),
        }
    }
}

#[derive(Debug, Error)]
#[error("failed to update configuration")]
pub struct ConfigurationError;

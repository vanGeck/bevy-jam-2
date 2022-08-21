use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct LogConfig {
    pub level: String,
}

impl LogConfig {
    /// Loads the most relevant instance of `LogConfig`.
    ///
    /// If the `LogConfig` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `LogConfig::default()`).
    ///
    /// If the 'LogConfig' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> LogConfig {
        let override_file = get_config_override_dir().join("log.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("log.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> LogConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<LogConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the log config file from {:?}! Falling back to LogConfig::default(). Error: {:?}",
                    path, error
                );
            LogConfig::default()
        })
}

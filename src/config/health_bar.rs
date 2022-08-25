use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};
use crate::positioning::Coords;

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct HealthBarConfig {
    pub coords: Coords,
}

impl HealthBarConfig {
    /// Loads the most relevant instance of `HealthBarConfig`.
    ///
    /// If the `HealthBar` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `HealthBar::default()`).
    ///
    /// If the 'HealthBar' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> HealthBarConfig {
        let override_file = get_config_override_dir().join("health_bar.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("health_bar.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> HealthBarConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<HealthBarConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the config file from {:?}! Falling back to HealthBarConfig::default(). Error: {:?}",
                    path, error
                );
            HealthBarConfig::default()
        })
}

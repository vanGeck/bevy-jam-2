use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "6d3dff28-404e-4a7a-b386-df43b3464389"]
pub struct SimConfig {
    /// Time between sim ticks in milliseconds.
    pub duration_millis: u64,
    /// The probability that loot is dropped: between 0 and 1.
    pub max_depth: i32,
    pub chance_corridor: f32,
    pub chance_empty: f32,
    pub chance_fight: f32,
}

impl SimConfig {
    /// Loads the most relevant instance of `SimConfig`.
    ///
    /// If the `SimConfig` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `SimConfig::default()`).
    ///
    /// If the 'SimConfig' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> SimConfig {
        let override_file = get_config_override_dir().join("sim.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("sim.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> SimConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<SimConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the config file from {:?}! Falling back to SimConfig::default(). Error: {:?}",
                    path, error
                );
            SimConfig::default()
        })
}

use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};
use crate::game::combat::Enemy;

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "5286cf90-c4a5-40da-a6c7-1081af73d649"]
pub struct EnemiesData {
    pub enemies: Vec<Enemy>,
}

impl EnemiesData {
    /// Loads the most relevant instance of `EnemiesData`.
    ///
    /// If the `EnemiesData` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `EnemiesData::default()`).
    ///
    /// If the 'RecipesData' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> EnemiesData {
        let override_file = get_config_override_dir().join("enemies.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("enemies.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> EnemiesData {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<EnemiesData>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the enemies data file from {:?}! Falling back to EnemiesData::default(). Error: {:?}",
                    path, error
                );
            EnemiesData::default()
        })
}

use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};


#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct DungeonParams {
    pub max_depth: i32,
    pub chance_corridor: f32,
    pub chance_empty: f32,
    pub chance_fight: f32,
    pub chance_fight_easy: f32,
    pub chance_fight_hard: f32,
}

impl DungeonParams {
    /// Loads the most relevant instance of `DungeonParams`.
    ///
    /// If the `DungeonParams` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `DungeonParams::default()`).
    ///
    /// If the 'DungeonParams' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> DungeonParams {
        let override_file = get_config_override_dir().join("dungeon_params.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("dungeon_params.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> DungeonParams {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<DungeonParams>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the config file from {:?}! Falling back to DungeonParams::default(). Error: {:?}",
                    path, error
                );
            DungeonParams::default()
        })
}
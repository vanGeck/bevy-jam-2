use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};
use crate::game::dungeon_gen::LevelBlueprint;

#[derive(Serialize, Deserialize, Clone, Default)]
#[serde(deny_unknown_fields)]
pub struct DungeonBlueprint {
    pub levels: Vec<LevelBlueprint>,
}

impl DungeonBlueprint {
    /// Loads the most relevant instance of `DungeonBlueprint`.
    ///
    /// If the `DungeonBlueprint` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `DungeonBlueprint::default()`).
    ///
    /// If the 'DungeonBlueprint' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> DungeonBlueprint {
        let override_file = get_config_override_dir().join("dungeon_layout.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("dungeon_layout.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> DungeonBlueprint {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<DungeonBlueprint>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the config file from {:?}! Falling back to DungeonBlueprint::default(). Error: {:?}",
                    path, error
                );
            DungeonBlueprint::default()
        })
}

use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct DungeonTexts {
    pub enter_room: Vec<String>,
    pub corridor: Vec<String>,
    pub door: Vec<String>,
    pub searching_room: Vec<String>,
    // pub searching_monster: Vec<String>,
    pub found_loot: Vec<String>,
    pub found_nothing: Vec<String>,
    pub enemy_encounter: Vec<String>,
    pub combat_enemy_hit: Vec<String>,
    pub combat_hero_hit: Vec<String>,
    pub combat_no_resolution: Vec<String>,
    // pub combat_pause: Vec<String>,
    pub combat_enemy_died: Vec<String>,
    pub combat_hero_died: Vec<String>,
    pub entered_start_room: Vec<String>,
    pub entered_end_room: Vec<String>,
}

impl DungeonTexts {
    /// Loads the most relevant instance of `DungeonTexts`.
    ///
    /// If the `DungeonTexts` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `DungeonTexts::default()`).
    ///
    /// If the 'DungeonTexts' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> DungeonTexts {
        let override_file = get_config_override_dir().join("dungeon_texts.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("dungeon_texts.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> DungeonTexts {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<DungeonTexts>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the config file from {:?}! Falling back to DungeonTexts::default(). Error: {:?}",
                    path, error
                );
            DungeonTexts::default()
        })
}

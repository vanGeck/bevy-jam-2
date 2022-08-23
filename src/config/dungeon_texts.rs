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
    // pub corridor: vec<str>,
    // pub door: vec<str>,
    // pub searching_room: vec<str>,
    // pub searching_monster: vec<str>,
    // pub found_loot: vec<str>,
    // pub found_nothing: vec<str>,
    // pub enemy_encounter: vec<str>,
    // pub combat_enemy_hit: vec<str>,
    // pub combat_hero_hit: vec<str>,
    // pub combat_no_resolution: vec<str>,
    // pub combat_pause: vec<str>,
    // pub combat_enemy_died: vec<str>,
    // pub combat_hero_died: vec<str>,
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
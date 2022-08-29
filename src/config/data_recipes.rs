use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};
use crate::game::recipes::Recipe;

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "866ad0fe-1aa3-4c05-af9a-434e887e796f"]
pub struct RecipesData {
    pub recipes: Vec<Recipe>,
}

impl RecipesData {
    /// Loads the most relevant instance of `RecipesData`.
    ///
    /// If the `RecipesData` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `RecipesData::default()`).
    ///
    /// If the 'RecipesData' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> RecipesData {
        let override_file = get_config_override_dir().join("recipes.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("recipes.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> RecipesData {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<RecipesData>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the recipes data file from {:?}! Falling back to RecipesData::default(). Error: {:?}",
                    path, error
                );
            RecipesData::default()
        })
}

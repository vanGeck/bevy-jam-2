use crate::game::ItemData;
use std::collections::HashMap;
use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct ItemsConfig {
    pub items: HashMap<String, ItemData>,
}

impl ItemsConfig {
    #[must_use]
    pub fn load_from_file() -> ItemsConfig {
        let override_file = get_config_override_dir().join("items.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("items.ron"))
        }
    }

    pub fn get_random_item(&self) -> &ItemData {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.items.len());
        let mut item_keys: Vec<&String> = self.items.keys().collect();
        item_keys.swap(0, index);
        let item_key = item_keys[0];
        self.items.get(item_key).unwrap().clone()
    }
}

fn load_from_path(path: &Path) -> ItemsConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<ItemsConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the items config file from {:?}! Falling back to ItemsConfig::default(). Error: {:?}",
                    path, error
                );
            ItemsConfig::default()
        })
}

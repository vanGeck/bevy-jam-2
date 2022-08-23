use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};

#[derive(Debug, Deserialize, Serialize, Default)]
#[serde(deny_unknown_fields)]
pub struct DebugConfig {
    /// Filters logs using the [`EnvFilter`] format
    pub log_filter: String,
    pub show_debug_window: bool,
    /// If true, the loader will bypass the menu and drop you straight into the game.
    /// Can be very handy for rapid testing, not having to click the play button every time.
    pub skip_straight_to_game: bool,
    pub launch_fullscreen: bool,
}

impl DebugConfig {
    /// Loads the most relevant instance of `DebugConfig`.
    ///
    /// If the `DebugConfig` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `DebugConfig::default()`).
    ///
    /// If the 'DebugConfig' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> DebugConfig {
        let override_file = get_config_override_dir().join("debug.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("debug.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> DebugConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<DebugConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the debug config file from {:?}! Falling back to DebugConfig::default(). Error: {:?}",
                    path, error
                );
            DebugConfig::default()
        })
}

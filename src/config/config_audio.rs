use std::fs;
use std::io::{Error, ErrorKind};
use std::path::Path;

use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::file_utils::{get_config_default_dir, get_config_override_dir};

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "d6b519cf-60bc-47e4-8570-1f40146932c5"]
pub struct AudioConfig {
    /// What volume the music should be played at. If this value is None, the music will not be
    /// played at all.
    /// The volume should be a value in the range [0.0, 1.0].
    pub music_volume: Option<f64>,
    /// What volume the sound effects should be played at. If this value is None, the music will
    /// not be played at all.
    /// The volume should be a value in the range [0.0, 1.0].
    pub sfx_volume: Option<f64>,
}

impl AudioConfig {
    /// Loads the most relevant instance of `AudioConfig`.
    ///
    /// If the `AudioConfig` override file exists, tries to load from config/override/ first. If that fails,
    /// log an error and use the Default trait implementation (ie: `AudioConfig::default()`).
    ///
    /// If the 'AudioConfig' override file does not exist, tries to load from config/default/ instead.
    #[must_use]
    pub fn load_from_file() -> AudioConfig {
        let override_file = get_config_override_dir().join("audio.ron");
        if override_file.exists() {
            load_from_path(&override_file)
        } else {
            load_from_path(&get_config_default_dir().join("audio.ron"))
        }
    }
}

fn load_from_path(path: &Path) -> AudioConfig {
    fs::read_to_string(path)
        .and_then(|data| ron::de::from_str::<AudioConfig>(&data).map_err(|error| Error::new(ErrorKind::Other, error)))
        .unwrap_or_else(|error| {
            error!(
                    "Failed to load the audio config file from {:?}! Falling back to AudioConfig::default(). Error: {:?}",
                    path, error
                );
            AudioConfig::default()
        })
}

#[derive(Default)]
pub struct AudioConfigLoader;

impl AssetLoader for AudioConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<AudioConfig>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["audio.ron"]
    }
}

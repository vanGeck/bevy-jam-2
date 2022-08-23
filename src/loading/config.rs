use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::game::{MusicId, SoundId, TextureId};
use serde::{Deserialize, Serialize};

/// This specifies all assets that must be loaded by the `LoadingState`.
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct LoadingConfig {
    pub textures: HashMap<TextureId, String>,
    pub atlases: HashMap<TextureId, String>,
    pub sound_effects: HashMap<SoundId, String>,
    pub music: HashMap<MusicId, String>,
}

impl LoadingConfig {
    /// Loads the LoadingConfig from file.
    #[must_use]
    pub fn load_from_file() -> LoadingConfig {
        let file = PathBuf::new().join("assets/config/loading.ron");
        let data = fs::read_to_string(file).expect("Unable to read loading config file");
        ron::de::from_str::<LoadingConfig>(&data).expect("Unable to deserialise LoadingConfig")
    }
}

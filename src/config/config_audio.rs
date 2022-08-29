use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

use serde::{Deserialize, Serialize};

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

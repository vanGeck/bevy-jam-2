use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

use serde::{Deserialize, Serialize};

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "6d3dff28-404e-4a7a-b386-df43b3464389"]
pub struct SimConfig {
    /// Time between sim ticks in milliseconds.
    pub duration_millis: u64,
    /// The probability that loot is dropped: between 0 and 1.
    pub max_depth: i32,
    pub chance_corridor: f32,
    pub chance_empty: f32,
    pub chance_fight: f32,
}

#[derive(Default)]
pub struct SimConfigLoader;

impl AssetLoader for SimConfigLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<SimConfig>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["sim.ron"]
    }
}

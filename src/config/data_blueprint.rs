use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

use serde::{Deserialize, Serialize};

use crate::game::dungeon_gen::LevelBlueprint;

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "53da3fb7-6cf8-4fbf-996e-b970037d0625"]
pub struct BlueprintData {
    pub levels: Vec<LevelBlueprint>,
}

#[derive(Default)]
pub struct BlueprintDataLoader;

impl AssetLoader for BlueprintDataLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<BlueprintData>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["blueprint.ron"]
    }
}

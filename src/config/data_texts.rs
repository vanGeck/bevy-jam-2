use std::collections::HashMap;

use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

use serde::{Deserialize, Serialize};

use crate::game::sim::dungeon_components::TextType;

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "9141eb1b-2cde-453a-8886-9757a7b17f9b"]
pub struct TextsData {
    pub map: HashMap<TextType, Vec<String>>,
}
#[derive(Default)]
pub struct TextsDataLoader;

impl AssetLoader for TextsDataLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<TextsData>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["texts.ron"]
    }
}

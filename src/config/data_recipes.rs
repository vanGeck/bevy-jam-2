use serde::{Deserialize, Serialize};

use crate::game::recipes::Recipe;
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "866ad0fe-1aa3-4c05-af9a-434e887e796f"]
pub struct RecipesData {
    pub recipes: Vec<Recipe>,
}

#[derive(Default)]
pub struct RecipesDataLoader;

impl AssetLoader for RecipesDataLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<RecipesData>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["recipes.ron"]
    }
}

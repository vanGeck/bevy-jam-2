// use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};
// use bevy::prelude::Vec2;
// use serde::{Deserialize, Serialize};
//
// use bevy::reflect::TypeUuid;
// use crate::game::TextureId;
//
// #[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
// #[serde(deny_unknown_fields)]
// #[uuid = "8aedb6db-6e11-4af1-89f5-69f87870b699"]
// pub struct AtlasPrefab {
//     pub texture: TextureId,
//     pub tile_size: Vec2,
//     pub columns: usize,
//     pub rows: usize,
// }
//
// #[derive(Default)]
// pub struct AtlasPrefabLoader;
//
// impl AssetLoader for AtlasPrefabLoader {
//     fn load<'a>(
//         &'a self,
//         bytes: &'a [u8],
//         load_context: &'a mut LoadContext,
//     ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
//         Box::pin(async move {
//             let custom_asset = ron::de::from_bytes::<AtlasPrefab>(bytes)?;
//             load_context.set_default_asset(LoadedAsset::new(custom_asset));
//             Ok(())
//         })
//     }
//
//     fn extensions(&self) -> &[&str] {
//         &["atlas.ron"]
//     }
// }

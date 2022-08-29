use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::game::items::{Item, ItemId};
use crate::positioning::Dimens;

use bevy::reflect::TypeUuid;
#[derive(Debug, Deserialize, Serialize, Default, Clone, TypeUuid)]
#[serde(deny_unknown_fields)]
#[uuid = "e739c4a0-e8b8-4773-9b3e-7e022c3c4f85"]
pub struct ItemsData {
    pub items: Vec<(Dimens, Item)>,
}
use bevy::asset::{AssetLoader, BoxedFuture, LoadContext, LoadedAsset};

impl ItemsData {
    pub fn get_random_item(&self) -> (Dimens, Item) {
        let mut rng = rand::thread_rng();
        let index = rng.gen_range(0..self.items.len());
        self.items.get(index).unwrap().clone()
    }

    pub fn try_get_item(&self, item_id: ItemId) -> Option<(Dimens, Item)> {
        self.items
            .iter()
            .find(|(_, item)| item.id == item_id)
            .cloned()
    }
}
#[derive(Default)]
pub struct ItemsDataLoader;

impl AssetLoader for ItemsDataLoader {
    fn load<'a>(
        &'a self,
        bytes: &'a [u8],
        load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<(), bevy::asset::Error>> {
        Box::pin(async move {
            let custom_asset = ron::de::from_bytes::<ItemsData>(bytes)?;
            load_context.set_default_asset(LoadedAsset::new(custom_asset));
            Ok(())
        })
    }

    fn extensions(&self) -> &[&str] {
        &["items.ron"]
    }
}

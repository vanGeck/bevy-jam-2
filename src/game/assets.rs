use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

/*
 Put all game assets here. If there's too many, create structs for each category.
 */

#[derive(AssetCollection)]
pub struct AssetHandles {
    #[asset(path = "placeholder.png")]
    pub placeholder: Handle<Image>
}
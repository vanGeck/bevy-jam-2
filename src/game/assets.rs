use bevy::prelude::*;
use bevy_asset_loader::asset_collection::AssetCollection;

/*
 Put all game assets here. If there's too many, create structs for each category.
 */

#[derive(AssetCollection)]
pub struct AssetHandles {
    #[asset(path = "placeholder.png")]
    pub placeholder: Handle<Image>,

    #[asset(path = "sprites/1x1_coin.png")]
    pub one_x_one_coin: Handle<Image>,

    #[asset(path = "sprites/green_square.png")]
    pub green_square: Handle<Image>,

    #[asset(path = "sprites/red_square.png")]
    pub red_square: Handle<Image>,

    #[asset(path = "sprites/selection_square.png")]
    pub selection_square: Handle<Image>,

    #[asset(path = "sprites/yellow_square.png")]
    pub yellow_square: Handle<Image>,
}


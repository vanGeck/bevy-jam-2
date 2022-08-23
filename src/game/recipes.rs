use bevy::prelude::*;

use crate::game::items::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Recipe {
    pub result: ItemId,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Component, Debug, Deserialize, Serialize)]
pub struct Ingredient {
    pub item_id: ItemId,
    pub quantity: i32,
}

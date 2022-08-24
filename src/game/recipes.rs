use crate::game::items::ItemId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Recipe {
    pub result: ItemId,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Ingredient {
    pub item_id: ItemId,
    pub quantity: i32,
}

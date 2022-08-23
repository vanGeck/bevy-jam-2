use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::TextureId;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub texture_id: TextureId,
    pub wearable: Option<EquipmentSlot>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum ItemId {
    Croissant,
    Athelas,
    HealthPotion,
    Vial,
    TurtleHerb,
    CandleStick,
    EmptyLantern,
    FilledLantern,
    LitLantern,
    FireEssence,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum EquipmentSlot {
    Weapon,
    Shield,
    Armour,
    Helmet,
    Necklace,
}

#[derive(Component)]
pub struct Equipment {
    pub slot: EquipmentSlot,
}

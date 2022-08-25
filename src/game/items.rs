use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::TextureId;
use crate::positioning::Pos;

// Marker Component
#[derive(Component)]
pub struct CraftItem;

#[derive(Component, Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub description: String,
    pub texture_id: TextureId,
    /// If this is an item that can be worn by the hero, which slot is it in and what is the
    /// offset in the equipment grid?
    pub wearable: Option<(EquipmentSlot, Pos)>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
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
    MediumShield,
}

#[derive(Serialize, Deserialize, Debug, Clone, Eq, PartialEq, Hash)]
pub enum EquipmentSlot {
    Helmet,
    Necklace,
    Armour,
    Weapon,
    Shield,
}

#[derive(Component)]
pub struct Equipment {
    pub slot: EquipmentSlot,
}

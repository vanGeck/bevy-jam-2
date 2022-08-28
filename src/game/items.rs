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
    pub stat_bonuses: Option<StatBonus>,
}

impl Default for Item {
    fn default() -> Self {
        Item {
            id: ItemId::Vial,
            name: "[EmptyItem]".to_string(),
            description: "[EmptyDescription]".to_string(),
            texture_id: TextureId::NotFound,
            wearable: None,
            stat_bonuses: Default::default(),
        }
    }
}

#[derive(Default, Copy, Clone, Debug, Serialize, Deserialize)]
pub struct StatBonus {
    pub max_hp: i32,
    pub damage: i32,
    pub damage_res: i32,
    pub combat_prof: i32,
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
    // Actual items
    HerbRed,
    HerbGreen,
    HerbViolet,
    EssenceMight,
    EssenceVitality,
    EssenceAlacrity,
    FlaskHealing,
    FlaskStrength,
    FlaskSkill,
    FlaskToughness,
    SwordRusty,
    Sword,
    SwordMasterwork,
    SwordWounding,
    SwordMasterworkWounding,
    SwordSpeed,
    SwordMasterworkSpeed,
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

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::TextureId;
use crate::mouse::MouseInteractive;
use crate::positioning::Pos;

use super::combat::Hero;

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
    pub health: i32,
    pub max_health: i32,
    pub proficiency: i32,
    pub damage_bonus: i32,
    pub damage_res: i32,
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

pub fn consume_item(
    mut commands: Commands,
    mut hero: ResMut<Hero>,
    items: Query<(Entity, &Item, &MouseInteractive)>,
) {
    for (e, item, interactive) in items.iter() {
        if interactive.right_clicked {
            if let Some(stats) = item.stat_bonuses {
                hero.combat_stats.health = (hero.combat_stats.health + stats.health).clamp(0, hero.combat_stats.max_health);
                hero.combat_stats.max_health += stats.max_health;
                hero.combat_stats.proficiency += stats.proficiency;
                hero.combat_stats.damage_res += stats.damage_res;
                hero.combat_stats.damage_bonus += stats.damage_bonus;
            }
            commands.entity(e).despawn_recursive();
        }
    }
}

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::TextureId;
use crate::mouse::MouseInteractive;
use crate::positioning::{Coords, Pos};

use super::combat::Hero;
use super::item_info_system::MouseOverItemInfo;
use super::timed_effect::{apply_timed_modifier, TemporaryModifier};

/// Marker component. This item is currently in the crafting window.
#[derive(Component)]
pub struct CraftItem;

#[derive(Component)]
pub struct FallingItem {
    /// These are the Coords of the silhouette item that it's animating towards.
    pub coords: Coords,
    /// The translation that it comes from.
    pub source: Vec2,
    /// The translation that it is moving towards.
    pub target: Vec2,
    /// The timer describing the move.
    pub timer: Timer,
}

impl FallingItem {
    pub fn new(coords: Coords, source: Vec2, target: Vec2) -> Self {
        Self {
            coords,
            source,
            target,
            timer: Timer::from_seconds(1.5, false),
        }
    }
}

/// Marker component. This item is unavailable, and must be rendered as a dark silhouette.
#[derive(Component)]
pub struct Silhouette;

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
    pub temporary_effect: Option<TemporaryModifier>,
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
            temporary_effect: Default::default(),
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
    mut tooltips: Query<Entity, With<MouseOverItemInfo>>,
) {
    for (e, item, interactive) in items.iter() {
        if interactive.right_clicked {
            if let Some(stats) = item.stat_bonuses {
                hero.combat_stats.health = (hero.combat_stats.health + stats.health)
                    .clamp(0, hero.combat_stats.max_health);
                hero.combat_stats.max_health += stats.max_health;
                hero.combat_stats.proficiency += stats.proficiency;
                hero.combat_stats.damage_res += stats.damage_res;
                hero.combat_stats.damage_bonus += stats.damage_bonus;

                commands.entity(e).despawn_recursive();
                if let Ok(tooltip) = tooltips.get_single_mut() {
                    commands.entity(tooltip).despawn_recursive();
                }
            }

            if let Some(modifier) = item.clone().temporary_effect {
                apply_timed_modifier(modifier, &mut commands);
                commands.entity(e).despawn_recursive();
                if let Ok(tooltip) = tooltips.get_single_mut() {
                    commands.entity(tooltip).despawn_recursive();
                }
            }
        }
    }
}

/// Apply a dark scrim to the item that is being dragged.
pub fn apply_silhouette(mut query: Query<(&mut Sprite, Option<&Silhouette>), With<Item>>) {
    for (mut sprite, silhouette) in query.iter_mut() {
        sprite.color = if silhouette.is_some() {
            Color::rgba(0.1, 0.1, 0.1, 1.)
        } else {
            Color::rgb(1., 1., 1.)
        };
    }
}

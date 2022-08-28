use bevy::prelude::*;
use crate::game::create_widget_hero::{HeroCurrentArmourDisplay, HeroCurrentShieldDisplay, HeroCurrentWeaponDisplay};
use crate::game::{EquipmentSlot, EquippedItem};

use super::{
    combat::Hero,
    create_widget_hero::{
        HeroCurrentHealthDisplay, HeroDamageBonusDisplay, HeroDamageResDisplay,
        HeroProficiencyDisplay,
    },
};

#[derive(Component)]
pub struct HealthBar;

pub fn update_health_bar(
    hero: ResMut<Hero>,
    mut health_bar_query: Query<&mut Transform, With<HealthBar>>,
) {
    if let Ok(mut transform) = health_bar_query.get_single_mut() {
        transform.scale.x = hero.combat_stats.health as f32 / hero.combat_stats.max_health as f32;
    }
}

pub fn update_hero_stats_display(
    hero: ResMut<Hero>,
    mut query: ParamSet<(
        Query<&mut Text, With<HeroProficiencyDisplay>>,
        Query<&mut Text, With<HeroDamageResDisplay>>,
        Query<&mut Text, With<HeroDamageBonusDisplay>>,
        Query<&mut Text, With<HeroCurrentHealthDisplay>>,
        Query<&mut Text, With<HeroCurrentArmourDisplay>>,
        Query<&mut Text, With<HeroCurrentShieldDisplay>>,
        Query<&mut Text, With<HeroCurrentWeaponDisplay>>,
    )>,
    equipped_items_query: Query<&EquippedItem>,
) {
    if let Ok(mut text) = query.p0().get_single_mut() {
        text.sections[0].value = format!("Combat Proficiency: {}", hero.combat_stats.proficiency);
    }
    if let Ok(mut text) = query.p1().get_single_mut() {
        text.sections[0].value = format!("Damage Resistance: {}", hero.combat_stats.damage_res);
    }
    if let Ok(mut text) = query.p2().get_single_mut() {
        text.sections[0].value = format!("Damage Bonus: {}", hero.combat_stats.damage_bonus);
    }
    if let Ok(mut text) = query.p3().get_single_mut() {
        text.sections[0].value = format!(
            "{}/{}",
            hero.combat_stats.health, hero.combat_stats.max_health
        );
    }
    if let Ok(mut text) = query.p4().get_single_mut() {
        for equipped_item in equipped_items_query.iter() {
            if equipped_item.slot == EquipmentSlot::Armour {
                text.sections[0].value = format!("Armour: {}", equipped_item.stat_bonus.damage_res);
            }
        }
    }
    if let Ok(mut text) = query.p5().get_single_mut() {
        for equipped_item in equipped_items_query.iter() {
            if equipped_item.slot == EquipmentSlot::Shield {
                text.sections[0].value = format!("Shield: {}", equipped_item.stat_bonus.damage_res);
            }
        }
    }
    if let Ok(mut text) = query.p6().get_single_mut() {
        for equipped_item in equipped_items_query.iter() {
            if equipped_item.slot == EquipmentSlot::Weapon {
                text.sections[0].value = format!("Weapon: {}", equipped_item.stat_bonus.damage_bonus);
            }
        }
    }
}

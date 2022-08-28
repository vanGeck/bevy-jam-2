use bevy::prelude::*;

use super::{
    combat::Hero,
    create_widget_hero::{HeroDamageBonusDisplay, HeroDamageResDisplay, HeroProficiencyDisplay},
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
    )>,
) {
    if let Ok(mut text) = query.p0().get_single_mut() {
        text.sections[0].value = format!("Proficiency: {}", hero.combat_stats.proficiency);
    }
    if let Ok(mut text) = query.p1().get_single_mut() {
        text.sections[0].value = format!("Damage Res: {}", hero.combat_stats.damage_res);
    }
    if let Ok(mut text) = query.p2().get_single_mut() {
        text.sections[0].value = format!("Damage Bonus: {}", hero.combat_stats.damage_bonus);
    }
}

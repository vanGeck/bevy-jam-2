use bevy::prelude::*;

use super::combat::Hero;

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

use bevy::prelude::*;

use crate::{
    config::health_bar::HealthBarConfig,
    positioning::{Coords, Depth},
};

use super::combat::Hero;

#[derive(Component)]
pub struct HealthBar {
    pub coords: Coords,
}

pub fn setup_health_bar(mut commands: Commands, config: Res<HealthBarConfig>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(255., 0.2, 0.2, 0.8),
                custom_size: Some(config.coords.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                config.coords.pos.x as f32 + config.coords.dimens.x as f32 * 0.5,
                config.coords.pos.y as f32 + config.coords.dimens.y as f32 * 0.5 + 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("HealthBar"))
        .insert(HealthBar {
            coords: config.coords,
        });

    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(config.coords.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                config.coords.pos.x as f32 + config.coords.dimens.x as f32 * 0.5,
                config.coords.pos.y as f32 + config.coords.dimens.y as f32 * 0.5 + 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("HealthBarBackground"));
}

// goes straight to 0 somehow....
pub fn update_health_bar(
    hero: ResMut<Hero>,
    mut health_bar_query: Query<&mut HealthBar>,
) {
    if let Ok(mut health_bar) = health_bar_query.get_single_mut() {
        debug!("before: {:?}", health_bar.coords.dimens.x);
        health_bar.coords.dimens.x *= hero.combat_stats.health / hero.combat_stats.max_health;
        debug!("after: {:?}", health_bar.coords.dimens.x);
    }
}

use bevy::prelude::*;

use crate::{
    config::health_bar::HealthBarConfig,
    positioning::{Coords, Depth, Dimens, Pos},
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

// the sprite custome size refusing to change...
pub fn update_health_bar(
    hero: ResMut<Hero>,
    mut health_bar_query: Query<(&mut Sprite, &mut Transform, &Coords), With<HealthBar>>,
) {
    if let Ok((mut sprite, _, coords)) = health_bar_query.get_single_mut() {
        sprite.custom_size = Some(Vec2::new(
                coords.dimens.x as f32 * (hero.combat_stats.health / hero.combat_stats.max_health) as f32,
                0.
            )
        );
    }
}

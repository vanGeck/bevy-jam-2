use bevy::prelude::*;

use crate::config::config_grid::GridConfig;
use crate::game::{AssetStorage, CleanupOnGameplayEnd};
use crate::positioning::depth::Depth;
use crate::positioning::dimens::Dimens;
use crate::positioning::grid::{Grid, GridCell};

pub fn spawn_playing_field(mut commands: Commands, config: Res<GridConfig>) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(config.coords.dimens.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                config.coords.pos.x as f32 + config.coords.dimens.x as f32 * 0.5,
                config.coords.pos.y as f32 + config.coords.dimens.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Grid::default())
        .insert(CleanupOnGameplayEnd)
        .with_children(|grid| {
            for y in 0..config.coords.dimens.y {
                for x in 0..config.coords.dimens.x {
                    grid.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.8, 0.8, 0.8, 0.5),
                            custom_size: Some(Dimens::unit().as_vec2()),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 + 0.5) - (config.coords.dimens.x as f32 * 0.5),
                            (y as f32 + 0.5) - (config.coords.dimens.y as f32 * 0.5),
                            1., // Relative to parent grid.
                        )
                        .with_scale(Vec3::new(0.9, 0.9, 1.)),
                        ..default()
                    })
                    .insert(GridCell::default());
                }
            }
        });
}

pub fn spawn_crafting_grid(
    mut _commands: Commands,
    _assets: Res<AssetStorage>,
    _config: Res<GridConfig>,
) {
    // TODO:...
}

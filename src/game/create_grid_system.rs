use bevy::prelude::*;

use crate::config::config_grid::GridConfig;
use crate::game::CleanupOnGameplayEnd;
use crate::positioning::coordinates::Coordinates;
use crate::positioning::depth::Depth;
use crate::positioning::dimensions::Dimensions;
use crate::positioning::grid::{Grid, GridCell};

pub fn create_grids(mut commands: Commands, config: Res<GridConfig>) {
    create_grid(&mut commands, &config.equipment);
    create_grid(&mut commands, &config.crafting);
}

fn create_grid(commands: &mut Commands, coordinates: &Coordinates) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                color: Color::rgba(0.2, 0.2, 0.2, 0.8),
                custom_size: Some(coordinates.dimensions.as_vec2()),
                ..default()
            },
            transform: Transform::from_xyz(
                coordinates.position.x as f32 + coordinates.dimensions.x as f32 * 0.5,
                coordinates.position.y as f32 + coordinates.dimensions.y as f32 * 0.5,
                Depth::Grid.z(),
            ),
            ..default()
        })
        .insert(Name::new("InventoryGrid"))
        .insert(Grid::default())
        .insert(CleanupOnGameplayEnd)
        .with_children(|grid| {
            for y in 0..coordinates.dimensions.y {
                for x in 0..coordinates.dimensions.x {
                    grid.spawn_bundle(SpriteBundle {
                        sprite: Sprite {
                            color: Color::rgba(0.8, 0.8, 0.8, 0.5),
                            custom_size: Some(Dimensions::unit().as_vec2()),
                            ..default()
                        },
                        transform: Transform::from_xyz(
                            (x as f32 + 0.5) - (coordinates.dimensions.x as f32 * 0.5),
                            (y as f32 + 0.5) - (coordinates.dimensions.y as f32 * 0.5),
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

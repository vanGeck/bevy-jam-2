use crate::game::assets::AssetHandles;
use crate::game::components::{Coordinate, Grid};
use bevy::prelude::*;

pub const GRID_WIDTH: i32 = 32;
pub const GRID_HEIGHT: i32 = 16;
pub const TILE_SIZE: i32 = 16;
pub const GRID_X_OFFSET: i32 = 5;
pub const GRID_Y_OFFSET: i32 = 5;

pub fn create_grid_system(mut commands: Commands, assets: Res<AssetHandles>) {
    let grid_entity = commands.spawn().id();

    let mut tiles: Vec<Entity> = Vec::new();

    commands
        .entity(grid_entity)
        .insert(Grid {
            width: GRID_WIDTH,
            height: GRID_HEIGHT,
            occupied: vec![false; (GRID_WIDTH * GRID_HEIGHT) as usize],
        })
        .insert(Name::new("Grid"))
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(GRID_X_OFFSET as f32, GRID_Y_OFFSET as f32, 0.),
            ..Default::default()
        });

    for j in 0..GRID_HEIGHT {
        for i in 0..GRID_WIDTH {
            let index = xy_index(i, j);
            let tile_x_position = ((i - (GRID_WIDTH / 2)) * TILE_SIZE) as f32;
            let tile_y_position = ((j - (GRID_HEIGHT / 2)) * TILE_SIZE) as f32;

            let tile_entity = commands.spawn().id();
            commands
                .entity(tile_entity)
                .insert(Name::new("Tile"))
                .insert(Coordinate { x: i, y: j })
                .insert_bundle(SpriteBundle {
                    texture: assets.green_square.clone(),
                    transform: Transform::from_xyz(tile_x_position, tile_y_position, 0.),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent.spawn_bundle(SpriteBundle {
                        texture: assets.selection_square.clone(),
                        transform: Transform::from_xyz(0., 0., 1.),
                        ..Default::default()
                    });
                });

            tiles.push(tile_entity);
        }
    }

    commands.entity(grid_entity).push_children(&tiles);
}

pub fn xy_index(x: i32, y: i32) -> usize {
    ((y * GRID_WIDTH) + x) as usize
}

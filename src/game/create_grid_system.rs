use bevy::prelude::*;

use crate::config::config_grid::GridConfig;
use crate::game::assets::AssetHandles;
use crate::game::components::Grid;
use crate::grid::coords::Coords;
use crate::grid::dimens::Dimens;
use crate::grid::pos::Pos;

pub fn create_grid_system(
    mut commands: Commands,
    assets: Res<AssetHandles>,
    config: Res<GridConfig>,
) {
    let grid_entity = commands.spawn().id();

    let mut tiles: Vec<Entity> = Vec::new();

    commands
        .entity(grid_entity)
        .insert(Grid {
            width: config.dimens.x,
            height: config.dimens.y,
            occupied: vec![false; (config.dimens.x * config.dimens.y) as usize],
        })
        .insert(Name::new("Grid"))
        .insert_bundle(SpatialBundle {
            transform: Transform::from_xyz(config.pos.x as f32, config.pos.y as f32, 0.),
            ..Default::default()
        });

    for j in 0..config.dimens.y {
        for i in 0..config.dimens.x {
            let index = xy_index(&config, i, j);
            let tile_x_position = ((i - (config.dimens.x / 2)) * config.tile_size) as f32;
            let tile_y_position = ((j - (config.dimens.y / 2)) * config.tile_size) as f32;

            let tile_entity = commands.spawn().id();
            commands
                .entity(tile_entity)
                .insert(Name::new("Tile"))
                .insert(Coords::new(Pos::new(i, j), Dimens::unit()))
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

pub fn xy_index(config: &GridConfig, x: i32, y: i32) -> usize {
    ((y * config.dimens.x) + x) as usize
}

use super::game::components::Grid;
use crate::game::assets::AssetHandles;
use bevy::prelude::*;

pub fn draw_grid_system(
    mut Commands: Commands,
    assets: Res<AssetHandles>,
    mut query: Query<&mut Grid>,
) {
    let grid = query.single_mut();
}

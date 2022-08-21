use super::game::components::Grid;
use crate::game::AssetStorage;
use bevy::prelude::*;

pub fn draw_grid_system(
    mut commands: Commands,
    assets: Res<AssetStorage>,
    mut query: Query<&mut Grid>,
) {
    let grid = query.single_mut();
}

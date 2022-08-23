use bevy::prelude::*;

use super::coordinates::Coordinates;

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Available,
    Unavailable,
}

#[derive(Component, Debug, Clone)]
pub struct Tile {
    index: usize,
    coords: Coordinates,
    tile_type: TileType,
}

impl Tile {
    pub fn new(index: usize, coords: Coordinates, tile_type: TileType) -> Self {
        Tile {
            index,
            coords,
            tile_type,
        }
    }
}

/// TODO: Do we really need this?
#[derive(Component, Default, Debug)]
pub struct Grid;

/// TODO: Do we really need this?
#[derive(Component, Default, Debug)]
pub struct GridCell;

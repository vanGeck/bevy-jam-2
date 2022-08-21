
use bevy::prelude::*;

use super::coords::Coords;

#[derive(Clone, Debug, PartialEq)]
pub enum TileType {
    Available,
    Unavailable,
}

#[derive(Component, Debug, Clone)]
pub struct Tile {
    index: usize,
    coords: Coords,
    tile_type: TileType,
}

impl Tile {
    pub fn new(index: usize, coords: Coords, tile_type: TileType) -> Self {
        Tile {
            index,
            coords,
            tile_type,
        }
    }
}

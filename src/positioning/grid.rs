use bevy::prelude::*;

use crate::positioning::Coords;

pub struct GridData {
    /// The translation offset of the grid.
    /// Add this to the Coords of items on the grid to get their translation.
    pub offset: Vec2,
    pub inventory: Coords,
    pub crafting: Coords,
}

impl GridData {
    /// Calculates the x component of the translation for a grid item with the given coords.
    pub fn calc_x(&self, coords: &Coords) -> f32 {
        self.offset.x + coords.pos.x as f32 + coords.dimens.x as f32 * 0.5
    }
    /// Calculates the y component of the translation for a grid item with the given coords.
    pub fn calc_y(&self, coords: &Coords) -> f32 {
        self.offset.y + coords.pos.y as f32 + coords.dimens.y as f32 * 0.5
    }
    /// Finds the center of the crafting grid.
    pub fn center_crafting(&self) -> Vec2 {
        Vec2::new(self.calc_x(&self.crafting), self.calc_y(&self.crafting))
    }
}

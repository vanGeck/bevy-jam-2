#![allow(dead_code)]

use crate::grid::dimens::Dimens;
use crate::grid::pos::Pos;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Can be used as a component for entities that are on the grid.
/// Contains a discrete position and dimensions.
#[derive(Component, Deserialize, Serialize, Default, Copy, Clone, Debug)]
pub struct Coords {
    /// The entity's discrete position. Not to be confused with its Transform, which is where the
    /// entity's sprite is actually at. Transform can be between squares, the discrete position
    /// is always at a square. The discrete position has it's coordinate in integral numbers,
    /// whereas the Transform's translation is in floats.
    ///
    /// If an entity is wider than 1 by 1, the pos is the bottom-left most tile in the entity's
    /// body.
    pub pos: Pos,
    /// Width and height of the entity.
    pub dimens: Dimens,
}

impl Coords {
    pub fn new(pos: Pos, dimens: Dimens) -> Self {
        Coords { pos, dimens }
    }

    pub fn overlaps(&self, other: &Coords) -> bool {
        self.pos.x < other.pos.x + other.dimens.x
            && self.pos.x + self.dimens.x > other.pos.x
            && self.pos.y < other.pos.y + other.dimens.y
            && self.pos.y + self.dimens.y > other.pos.y
    }
    pub fn overlaps_pos(&self, other: &Pos) -> bool {
        let other = Coords::new(*other, Dimens::new(1, 1));
        self.overlaps(&other)
    }
    pub fn overlaps_rect(&self, other: &Pos, other_dimens: &Dimens) -> bool {
        let other = Coords::new(*other, *other_dimens);
        self.overlaps(&other)
    }

    /// Converts the given discrete position to a translation, taking into account the dimensions
    /// of the entity.
    ///
    /// The discrete position is the bottom-left corner of the entity, a translation is the
    /// center point of the entity.
    pub fn to_centered_coords(&self, pos: Pos) -> (f32, f32) {
        (
            pos.x as f32 + 0.5 * self.dimens.x as f32,
            pos.y as f32 + 0.5 * self.dimens.y as f32,
        )
    }

    /// Converts the given translation, which is the center-point of the entity, into a pair of
    /// anchored coordinates, describing the bottom-left corner of the entity.
    ///
    /// Note that this does NOT return a discrete position: output is not rounded or floored.
    pub fn to_anchor_coords(&self, transform: &Transform) -> (f32, f32) {
        (
            transform.translation.x - 0.5 * self.dimens.x as f32,
            transform.translation.y - 0.5 * self.dimens.y as f32,
        )
    }
}

#![allow(dead_code)]

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::positioning::dimensions::Dimensions;
use crate::positioning::position::Position;

/// Can be used as a component for entities that are on the grid.
/// Contains a discrete position and dimensions.
#[derive(Component, Deserialize, Serialize, Default, Copy, Clone, Debug)]
pub struct Coordinates {
    /// The entity's discrete position. Not to be confused with its Transform, which is where the
    /// entity's sprite is actually at. Transform can be between squares, the discrete position
    /// is always at a square. The discrete position has it's coordinate in integral numbers,
    /// whereas the Transform's translation is in floats.
    ///
    /// If an entity is wider than 1 by 1, the position is the bottom-left most tile in the entity's
    /// body.
    pub position: Position,
    /// Width and height of the entity.
    pub dimensions: Dimensions,
}

impl Coordinates {
    pub fn new(position: Position, dimensions: Dimensions) -> Self {
        Coordinates {
            position,
            dimensions,
        }
    }

    /// Return true iff the two Coords rectangles overlap at all.
    pub fn overlaps(&self, other: &Coordinates) -> bool {
        self.position.x < other.position.x + other.dimensions.x
            && self.position.x + self.dimensions.x > other.position.x
            && self.position.y < other.position.y + other.dimensions.y
            && self.position.y + self.dimensions.y > other.position.y
    }
    pub fn overlaps_pos(&self, other: &Position) -> bool {
        let other = Coordinates::new(*other, Dimensions::new(1, 1));
        self.overlaps(&other)
    }
    pub fn overlaps_rect(&self, other: &Position, other_dimens: &Dimensions) -> bool {
        let other = Coordinates::new(*other, *other_dimens);
        self.overlaps(&other)
    }
    /// Return true iff the given 'other' Coords is completely enclosed by this Coords.
    /// Two Coords that are equal will enclose each other.
    pub fn encloses(&self, other: &Coordinates) -> bool {
        self.position.x <= other.position.x
            && self.position.x + self.dimensions.x >= other.position.x + other.dimensions.x
            && self.position.y <= other.position.y
            && self.position.y + self.dimensions.y >= other.position.y + other.dimensions.y
    }

    /// Converts the given discrete position to a translation, taking into account the dimensions
    /// of the entity.
    ///
    /// The discrete position is the bottom-left corner of the entity, a translation is the
    /// center point of the entity.
    pub fn to_centered_coords(self, pos: Position) -> (f32, f32) {
        (
            pos.x as f32 + 0.5 * self.dimensions.x as f32,
            pos.y as f32 + 0.5 * self.dimensions.y as f32,
        )
    }

    /// Converts the given translation, which is the center-point of the entity, into a pair of
    /// anchored coordinates, describing the bottom-left corner of the entity.
    ///
    /// Note that this does NOT return a discrete position: output is not rounded or floored.
    pub fn to_anchor_coords(self, transform: &Transform) -> (f32, f32) {
        (
            transform.translation.x - 0.5 * self.dimensions.x as f32,
            transform.translation.y - 0.5 * self.dimensions.y as f32,
        )
    }
}

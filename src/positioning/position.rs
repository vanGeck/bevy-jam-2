#![allow(dead_code)]

use std::cmp::Ordering;
use std::ops::{Add, Deref, DerefMut, Sub};

use bevy::prelude::{IVec2, Vec2};
use serde::{Deserialize, Serialize};

/// Defines a set of discrete grid coordinates.
/// Wraps an IVec2, which works well together with bevy.
#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Position(IVec2);

impl Position {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Position(IVec2::new(x, y))
    }

    #[must_use]
    pub fn plus_x(self, x: i32) -> Self {
        Position::new(self.x + x, self.y)
    }

    #[must_use]
    pub fn plus_y(self, y: i32) -> Self {
        Position::new(self.x, self.y + y)
    }

    /// If you have got another Pos, consider simply adding them together using the + operator.
    #[must_use]
    pub fn plus_xy(self, x: i32, y: i32) -> Self {
        Position::new(self.x + x, self.y + y)
    }
}

impl Deref for Position {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Position {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Order by x first, then y.
///
/// This is implemented purely to make it possible to save level and adventure files in a
/// deterministic way.
impl PartialOrd<Self> for Position {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.x < other.x {
            Some(Ordering::Less)
        } else if self.x > other.x {
            Some(Ordering::Greater)
        } else if self.y < other.y {
            Some(Ordering::Less)
        } else if self.y > other.y {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

impl Ord for Position {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, other: Position) -> Position {
        Position::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, other: Position) -> Position {
        Position::new(self.x + other.x, self.y + other.y)
    }
}

impl From<Vec2> for Position {
    fn from(vec: Vec2) -> Self {
        Position::new(vec.x.floor() as i32, vec.y.floor() as i32)
    }
}

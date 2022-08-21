use std::cmp::Ordering;
use std::ops::{Add, Deref, DerefMut, Sub};

use bevy::prelude::IVec2;
use serde::{Deserialize, Serialize};

/// Defines a set of discrete grid coordinates.
/// Wraps an IVec2, which works well together with bevy.
#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Pos(IVec2);

impl Pos {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Pos(IVec2::new(x, y))
    }

    #[must_use]
    pub fn append_x(self, x: i32) -> Self {
        Pos::new(self.x + x, self.y)
    }

    #[must_use]
    pub fn append_y(self, y: i32) -> Self {
        Pos::new(self.x, self.y + y)
    }

    #[must_use]
    pub fn append_xy(self, x: i32, y: i32) -> Self {
        Pos::new(self.x + x, self.y + y)
    }
}

impl Deref for Pos {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Pos {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Order by x first, then y.
///
/// This is implemented purely to make it possible to save level and adventure files in a
/// deterministic way.
impl PartialOrd<Self> for Pos {
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

impl Ord for Pos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Pos {
    type Output = Pos;

    fn sub(self, other: Pos) -> Pos {
        Pos::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, other: Pos) -> Pos {
        Pos::new(self.x + other.x, self.y + other.y)
    }
}

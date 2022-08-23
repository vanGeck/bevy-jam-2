#![allow(dead_code)]

use std::cmp::Ordering;
use std::ops::{Add, Deref, DerefMut, Sub};

use bevy::prelude::IVec2;
use serde::{Deserialize, Serialize};

/// Defines a width and height in terms of discrete grid coordinates.
/// Wraps an IVec2, which works well together with bevy.
#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dimensions(pub IVec2);

impl Dimensions {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Dimensions(IVec2::new(x, y))
    }

    #[must_use]
    pub fn unit() -> Self {
        Dimensions(IVec2::new(1, 1))
    }

    #[must_use]
    pub fn plus_x(self, x: i32) -> Self {
        Dimensions::new(self.x + x, self.y)
    }

    #[must_use]
    pub fn plus_y(self, y: i32) -> Self {
        Dimensions::new(self.x, self.y + y)
    }

    /// If you have got another Dimens, consider simply adding them together using the + operator.
    #[must_use]
    pub fn plus_xy(self, x: i32, y: i32) -> Self {
        Dimensions::new(self.x + x, self.y + y)
    }
}

impl Deref for Dimensions {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dimensions {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Order by x first, then y.
impl PartialOrd<Self> for Dimensions {
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

impl Ord for Dimensions {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Dimensions {
    type Output = Dimensions;

    fn sub(self, other: Dimensions) -> Dimensions {
        Dimensions::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Dimensions {
    type Output = Dimensions;

    fn add(self, other: Dimensions) -> Dimensions {
        Dimensions::new(self.x + other.x, self.y + other.y)
    }
}

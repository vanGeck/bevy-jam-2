use std::cmp::Ordering;
use std::ops::{Add, Deref, DerefMut, Sub};

use bevy::prelude::IVec2;
use serde::{Deserialize, Serialize};

/// Defines a width and height in terms of discrete grid coordinates.
/// Wraps an IVec2, which works well together with bevy.
#[derive(Deserialize, Serialize, Default, Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Dimens(IVec2);

impl Dimens {
    #[must_use]
    pub fn new(x: i32, y: i32) -> Self {
        Dimens(IVec2::new(x, y))
    }

    #[must_use]
    pub fn unit() -> Self {
        Dimens(IVec2::new(1, 1))
    }

    #[must_use]
    pub fn plus_x(self, x: i32) -> Self {
        Dimens::new(self.x + x, self.y)
    }

    #[must_use]
    pub fn plus_y(self, y: i32) -> Self {
        Dimens::new(self.x, self.y + y)
    }

    /// If you have got another Dimens, consider simply adding them together using the + operator.
    #[must_use]
    pub fn plus_xy(self, x: i32, y: i32) -> Self {
        Dimens::new(self.x + x, self.y + y)
    }
}

impl Deref for Dimens {
    type Target = IVec2;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Dimens {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Order by x first, then y.
impl PartialOrd<Self> for Dimens {
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

impl Ord for Dimens {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Sub for Dimens {
    type Output = Dimens;

    fn sub(self, other: Dimens) -> Dimens {
        Dimens::new(self.x - other.x, self.y - other.y)
    }
}

impl Add for Dimens {
    type Output = Dimens;

    fn add(self, other: Dimens) -> Dimens {
        Dimens::new(self.x + other.x, self.y + other.y)
    }
}

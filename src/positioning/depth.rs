#![allow(dead_code, unreachable_patterns)]

/// Use this to arrange entities along the z-axis.
pub enum Depth {
    Background,
    Grid,
    Item,
    /// When player is dragging an item, it appears above normal items.
    FloatingItem,
    Cursor,
    /// Some decorative foreground stuff.
    Foreground,
    Particle,
    Menu,
}

impl Depth {
    pub fn z(self) -> f32 {
        match self {
            Depth::Background => 0.,
            Depth::Grid => 100.,
            Depth::Item => 200.,
            Depth::FloatingItem => 300.,
            Depth::Cursor => 400.,
            Depth::Foreground => 500.,
            Depth::Particle => 600.,
            Depth::Menu => 700.,
        }
    }
}

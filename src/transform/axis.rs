//! Axis representation.

use crate::cartesian;

/// Represents any of the 3 axis in a three-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl From<Axis> for cartesian::Coordinates {
    fn from(axis: Axis) -> Self {
        match axis {
            Axis::X => cartesian::Coordinates::default().with_x(1.),
            Axis::Y => cartesian::Coordinates::default().with_y(1.),
            Axis::Z => cartesian::Coordinates::default().with_z(1.),
        }
    }
}

impl Axis {
    /// Returns the axis as a [cartesian coordinate](cartesian::Coordinates).
    pub fn as_cartesian(&self) -> cartesian::Coordinates {
        (*self).into()
    }
}

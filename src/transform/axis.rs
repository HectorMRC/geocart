//! Axis representation.

use num_traits::One;

use crate::cartesian;

/// Represents any of the 3 axis in a three-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl<T> From<Axis> for cartesian::Coordinates<T>
where
    T: Default + One,
{
    fn from(axis: Axis) -> Self {
        match axis {
            Axis::X => cartesian::Coordinates::default().with_x(T::one()),
            Axis::Y => cartesian::Coordinates::default().with_y(T::one()),
            Axis::Z => cartesian::Coordinates::default().with_z(T::one()),
        }
    }
}

impl Axis {
    /// Returns the axis as a [cartesian coordinate](cartesian::Coordinates).
    pub fn as_cartesian<T>(&self) -> cartesian::Coordinates<T>
    where
        T: Default + One,
    {
        (*self).into()
    }
}

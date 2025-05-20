//! Axis representation.

use num_traits::One;

use crate::cartesian::Cartesian;

/// Represents any of the 3 axis in a three-dimensional space.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

impl<T> From<Axis> for Cartesian<T>
where
    T: Default + One,
{
    fn from(axis: Axis) -> Self {
        match axis {
            Axis::X => Cartesian::default().with_x(T::one()),
            Axis::Y => Cartesian::default().with_y(T::one()),
            Axis::Z => Cartesian::default().with_z(T::one()),
        }
    }
}

impl Axis {
    /// Returns the axis as a [cartesian coordinate](cartesian::Cartesian).
    pub fn as_cartesian<T>(&self) -> Cartesian<T>
    where
        T: Default + One,
    {
        (*self).into()
    }
}

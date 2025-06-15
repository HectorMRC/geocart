//! Positive definition.

use std::ops::Add;

use num_traits::{Signed, Zero};

/// A value that is always positive.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Positive<T>(T);

impl<T> From<T> for Positive<T>
where
    T: Signed,
{
    fn from(value: T) -> Self {
        Self(value.abs())
    }
}

impl<T> Add<Self> for Positive<T>
where
    T: Signed + Add<T, Output = T>,
{
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from(self.0 + rhs.0)
    }
}

impl<T> Zero for Positive<T>
where
    T: Signed + Zero,
{
    fn zero() -> Self {
        Self(T::zero())
    }

    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl<T> Eq for Positive<T> where T: PartialEq {}

impl<T> Positive<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

//! Positive float definition.

use num_traits::Signed;

/// A [`Float`] that is always positive.
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PositiveFloat<T>(T);

impl<T> From<T> for PositiveFloat<T>
where
    T: Signed,
{
    fn from(value: T) -> Self {
        Self(value.abs())
    }
}

impl<T> Eq for PositiveFloat<T> where T: PartialEq {}

impl<T> PositiveFloat<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

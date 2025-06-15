//! Positive definition.

use num_traits::Signed;

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

impl<T> Eq for Positive<T> where T: PartialEq {}

impl<T> Positive<T> {
    /// Returns the inner value.
    pub fn into_inner(self) -> T {
        self.0
    }
}

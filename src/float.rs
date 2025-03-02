//! Positive float definition.

#[cfg(not(feature = "f64"))]
pub(crate) type Float = f32;

#[cfg(not(feature = "f64"))]
pub(crate) use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[cfg(feature = "f64")]
pub(crate) type Float = f64;

#[cfg(feature = "f64")]
pub(crate) use std::f64::consts::{FRAC_PI_2, PI, TAU};

/// A [`Float`] that is always positive.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PositiveFloat(Float);

impl From<Float> for PositiveFloat {
    fn from(value: Float) -> Self {
        Self(value.abs())
    }
}

impl From<PositiveFloat> for Float {
    fn from(value: PositiveFloat) -> Self {
        value.0
    }
}

impl Eq for PositiveFloat {}

impl Ord for PositiveFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.total_cmp(&other.0)
    }
}

impl PartialOrd for PositiveFloat {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PositiveFloat {
    /// Smallest positive [`Float`] value.
    pub const MIN: Self = Self(0.);
    /// Largest finite [`Float`] value.
    pub const MAX: Self = Self(Float::MAX);

    /// Returns the value as a [`Float`].
    pub fn as_float(self) -> Float {
        self.0
    }
}

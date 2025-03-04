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
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct PositiveFloat(Float);

impl From<Float> for PositiveFloat {
    fn from(value: Float) -> Self {
        Self(value.abs())
    }
}

impl Eq for PositiveFloat {}

impl PositiveFloat {
    /// Smallest positive [`Float`] value.
    pub const MIN: Self = Self(0.);

    /// Returns the value as a [`Float`].
    pub fn as_float(self) -> Float {
        self.0
    }
}

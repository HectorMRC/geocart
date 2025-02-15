pub mod cartesian;
pub mod geographic;
pub mod projection;

#[cfg(not(feature = "f64"))]
pub(self) type Float = f32;
#[cfg(not(feature = "f64"))]
pub(self) use std::f32::consts::{FRAC_PI_2, PI, TAU};

#[cfg(feature = "f64")]
pub(self) type Float = f64;
#[cfg(feature = "f64")]
pub(self) use std::f64::consts::{FRAC_PI_2, PI, TAU};

#[cfg(test)]
mod tests {
    use std::ops::Sub;

    use num_traits::Signed;

    /// Returns true if, and only if, abs_error >= |v1 - v2|. Otherwise returns false.
    #[inline(always)]
    pub fn approx_eq<T, E>(v1: T, v2: T, abs_error: E) -> bool
    where
        T: Sub<Output = T> + Signed,
        E: PartialOrd<T>,
    {
        abs_error >= (v1 - v2).abs()
    }
}

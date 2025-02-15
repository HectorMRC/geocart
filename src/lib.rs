pub mod cartesian;
pub mod geographic;

#[cfg(not(feature = "f64"))]
pub type Float = f32;
#[cfg(not(feature = "f64"))]
pub use std::f32::consts::{FRAC_PI_2, PI};

#[cfg(feature = "f64")]
pub type Float = f64;
#[cfg(feature = "f64")]
pub use std::f64::consts::{FRAC_PI_2, PI};

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

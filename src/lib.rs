pub mod cartesian;
pub mod float;
pub mod geographic;
pub mod projection;
pub mod radian;
pub mod shape;
pub mod transform;

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

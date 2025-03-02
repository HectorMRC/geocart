//! Transform definitions and implementations.

mod axis;
pub use axis::*;

mod rotation;
pub use rotation::*;

/// A geometric transformation.
pub trait Transform<Rhs> {
    /// Performs the geometric over `Rhs`.
    fn transform(&self, point: Rhs) -> Rhs;
}

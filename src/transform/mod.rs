//! Transform definitions and implementations.

mod rotation;
pub use rotation::*;

/// A geometric transformation.
pub trait Transform<Rhs> {
    /// Performs the geometric transformation over `Rhs`.
    fn transform(&self, point: Rhs) -> Rhs;
}

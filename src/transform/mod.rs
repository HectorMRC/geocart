//! Transform definitions and implementations.

mod axis;
mod rotation;

pub use self::axis::Axis;
pub use self::rotation::Rotation;

/// A geometric transformation.
pub trait Transform<Rhs> {
    /// Performs the transformation over `Rhs`.
    fn transform(&self, point: Rhs) -> Rhs;
}

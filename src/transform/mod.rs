//! Transform definitions and implementations.

mod rotation;

pub use self::rotation::Rotation;

/// A geometric transformation.
pub trait Transform<Rhs> {
    /// Performs the transformation over `Rhs`.
    fn transform(&self, point: Rhs) -> Rhs;
}

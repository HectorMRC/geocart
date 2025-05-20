//! Projection definition and implementations.

mod equirectangular;
mod gall_stereographic;

use crate::{cartesian::Cartesian, geographic::Geographic};

pub use self::equirectangular::Equirectangular;
pub use self::gall_stereographic::GallStereographic;

/// A projection is a function that maps geographic coordinates to cartesian coordinates and vice
/// versa.
pub trait Projection<T> {
    /// Projects the given geographic coordinates to cartesian coordinates.
    fn forward(&self, coordinates: &Geographic<T>) -> Cartesian<T>;
    /// Unprojects the given cartesian coordinates to geographic coordinates.
    fn reverse(&self, coordinates: &Cartesian<T>) -> Geographic<T>;
}

//! Projection definition and implementations.

mod equirectangular;
pub use equirectangular::Equirectangular;

mod gall_stereographic;
pub use gall_stereographic::GallStereographic;

use crate::{cartesian, geographic};

/// A projection is a function that maps geographic coordinates to cartesian coordinates and vice versa.
pub trait Projection<T> {
    /// Projects the given geographic coordinates to cartesian coordinates.
    fn forward(&self, coordinates: &geographic::Coordinates<T>) -> cartesian::Coordinates<T>;
    /// Unprojects the given cartesian coordinates to geographic coordinates.
    fn reverse(&self, coordinates: &cartesian::Coordinates<T>) -> geographic::Coordinates<T>;
}

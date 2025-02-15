use crate::{cartesian, geographic, Float};

use super::Projection;

/// Gall Stereographic projection.
pub struct GallStereographic;

impl Projection for GallStereographic {
    fn forward(coordinates: &geographic::Coordinates) -> cartesian::Coordinates {
        cartesian::Coordinates {
            x: coordinates.longitude.inner() / Float::sqrt(2.),
            y: (1. + Float::sqrt(2.) / 2.) * (coordinates.latitude.inner() / 2.).tan(),
            ..Default::default()
        }
    }

    fn reverse(coordinates: &cartesian::Coordinates) -> geographic::Coordinates {
        geographic::Coordinates {
            latitude: (2. * (coordinates.y / (1. + Float::sqrt(2.) / 2.)).atan()).into(),
            longitude: (coordinates.x * Float::sqrt(2.)).into(),
            ..Default::default()
        }
    }
}

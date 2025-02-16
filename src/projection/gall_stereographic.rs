//! Gall Stereographic projection.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{cartesian, geographic, Float};

use super::Projection;

/// The [Gall Stereographic projection](https://en.wikipedia.org/wiki/Gall_stereographic_projection).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct GallStereographic {
    radius: Float,
}

impl Projection for GallStereographic {
    fn forward(&self, coordinates: &geographic::Coordinates) -> cartesian::Coordinates {
        cartesian::Coordinates {
            x: self.radius * coordinates.longitude.inner() / Float::sqrt(2.),
            y: self.radius
                * (1. + Float::sqrt(2.) / 2.)
                * (coordinates.latitude.inner() / 2.).tan(),
            ..Default::default()
        }
    }

    fn reverse(&self, coordinates: &cartesian::Coordinates) -> geographic::Coordinates {
        geographic::Coordinates {
            latitude: (2. * (coordinates.y / (self.radius * (1. + Float::sqrt(2.) / 2.))).atan())
                .into(),
            longitude: (coordinates.x * Float::sqrt(2.) / self.radius).into(),
            ..Default::default()
        }
    }
}

//! Gall Stereographic projection.

#[cfg(feature = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

use crate::{cartesian, geographic, Float, PositiveFloat};

use super::Projection;

/// The [Gall Stereographic projection](https://en.wikipedia.org/wiki/Gall_stereographic_projection).
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "wasm", wasm_bindgen)]
pub struct GallStereographic {
    radius: PositiveFloat,
}

impl Projection for GallStereographic {
    fn forward(&self, coordinates: &geographic::Coordinates) -> cartesian::Coordinates {
        cartesian::Coordinates {
            x: self.radius.as_float() * coordinates.longitude.as_float() / Float::sqrt(2.),
            y: self.radius.as_float()
                * (1. + Float::sqrt(2.) / 2.)
                * (coordinates.latitude.as_float() / 2.).tan(),
            ..Default::default()
        }
    }

    fn reverse(&self, coordinates: &cartesian::Coordinates) -> geographic::Coordinates {
        geographic::Coordinates {
            latitude: (2.
                * (coordinates.y / (self.radius.as_float() * (1. + Float::sqrt(2.) / 2.))).atan())
            .into(),
            longitude: (coordinates.x * Float::sqrt(2.) / self.radius.as_float()).into(),
            ..Default::default()
        }
    }
}
